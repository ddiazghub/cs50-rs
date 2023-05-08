use std::fs::File;
use std::{env, io};
use std::io::{Read, Write};

/// WAV file header size in bytes.
const HEADER_SIZE: usize = 44;

/// Integer byte endianness.
enum Endianness {
    Big,
    Little
}

/// Deserializes value from a stream of bytes.
trait FromBytes {
    /// Deserializes value from a stream of bytes.
    ///
    /// # Arguments
    /// * `bytes` - The element's byte representation.
    /// * `endianness` - Endianness of the bytes.
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self;
}

impl FromBytes for String {
    // Deserialize string from bytes.
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        match endianness {
            Endianness::Big => bytes.iter().map(|b| char::from(*b)).collect(),
            Endianness::Little => bytes.iter().rev().map(|b| char::from(*b)).collect()
        }
    }
}

impl FromBytes for u32 {
    // Deserialize u32 from bytes.
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        if bytes.len() != 4 {
            panic!("Bytes array should have a length of 4")
        }

        match endianness {
            Endianness::Big => ((bytes[0] as u32) << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + (bytes[4] as u32),
            Endianness::Little => ((bytes[3] as u32) << 24) + ((bytes[2] as u32) << 16) + ((bytes[1] as u32) << 8) + (bytes[0] as u32)
        }
    }
}

impl FromBytes for u16 {
    // Deserialize u16 from bytes.
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        if bytes.len() != 2 {
            panic!("Bytes array should have a length of 2")
        }

        match endianness {
            Endianness::Big => ((bytes[0] as u16) << 8) + (bytes[1] as u16),
            Endianness::Little => ((bytes[1] as u16) << 8) + (bytes[0] as u16)
        }
    }
}

/// A WAV file's header.
#[derive(Debug)]
struct WavAudioFileHeader {
    pub chunk_id: String,
    pub chunk_size: u32,
    pub format: String,
    pub sub_chunk1_id: String,
    pub sub_chunk1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub sub_chunk2_id: String,
    pub sub_chunk2_size: u32,
}

impl WavAudioFileHeader {
    /// Creates a new WAV header from a buffer of bytes.
    ///
    /// # Arguments
    /// * `bytes` - The byte buffer.
    pub fn new(bytes: &[u8; HEADER_SIZE]) -> Self {
        WavAudioFileHeader {
            chunk_id: String::from_bytes(&bytes[0..4], Endianness::Big),
            chunk_size: u32::from_bytes(&bytes[4..8], Endianness::Little),
            format: String::from_bytes(&bytes[8..12], Endianness::Big),
            sub_chunk1_id: String::from_bytes(&bytes[12..16], Endianness::Big),
            sub_chunk1_size: u32::from_bytes(&bytes[16..20], Endianness::Little),
            audio_format: u16::from_bytes(&bytes[20..22], Endianness::Little),
            num_channels: u16::from_bytes(&bytes[22..24], Endianness::Little),
            sample_rate: u32::from_bytes(&bytes[24..28], Endianness::Little),
            byte_rate: u32::from_bytes(&bytes[28..32], Endianness::Little),
            block_align: u16::from_bytes(&bytes[32..34], Endianness::Little),
            bits_per_sample: u16::from_bytes(&bytes[34..36], Endianness::Little),
            sub_chunk2_id: String::from_bytes(&bytes[36..40], Endianness::Big),
            sub_chunk2_size: u32::from_bytes(&bytes[40..44], Endianness::Little)
        }
    }

    /// Serializes a new WAV header to a buffer of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.chunk_id.as_bytes());
        bytes.extend_from_slice(&self.chunk_size.to_le_bytes());
        bytes.extend_from_slice(self.format.as_bytes());
        bytes.extend_from_slice(self.sub_chunk1_id.as_bytes());
        bytes.extend_from_slice(&self.sub_chunk1_size.to_le_bytes());
        bytes.extend_from_slice(&self.audio_format.to_le_bytes());
        bytes.extend_from_slice(&self.num_channels.to_le_bytes());
        bytes.extend_from_slice(&self.sample_rate.to_le_bytes());
        bytes.extend_from_slice(&self.byte_rate.to_le_bytes());
        bytes.extend_from_slice(&self.block_align.to_le_bytes());
        bytes.extend_from_slice(&self.bits_per_sample.to_le_bytes());
        bytes.extend_from_slice(&self.sub_chunk2_id.as_bytes());
        bytes.extend_from_slice(&self.sub_chunk2_size.to_le_bytes());
        bytes
    }
}

/// A 16 bit WAV audio file.
///
/// # Arguments
/// * `header` - The file's header.
/// * `data` - The audio samples.
struct WavAudioFile16 {
    pub header: WavAudioFileHeader,
    pub data: Vec<i16>
}

impl WavAudioFile16 {
    /// Reads a WAV audio file and creates a new instance of the current struct.
    ///
    /// # Arguments
    /// * `filename` - The file's name.
    pub fn new(filename: &str) -> io::Result<Self> {
        if WavAudioFile16::is_wav_filename(filename) {
            let mut file = File::open(filename)?;
            let mut header_bytes: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
            file.read(&mut header_bytes)?;
            let header = WavAudioFileHeader::new(&header_bytes);

            if header.bits_per_sample != 16 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "The file is not a 16 bit WAV file"));
            }

            let mut buffer = vec![0; header.sub_chunk2_size as usize];
            file.read(&mut buffer)?;
            let mut data: Vec<i16> = Vec::new();

            for i in (0..buffer.len()).step_by(2) {
                data.push(i16::from_le_bytes([buffer[i], buffer[i + 1]]))
            }

            Ok(WavAudioFile16 {
                header,
                data
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "File should be a wav file"))
        }
    }

    /// Changes the file's volume by multiplying the value of each sample by a scale value.
    /// Writes the result to an output file.
    ///
    /// # Arguments
    /// * `out` - The output file.
    /// * `scale` - The value used to scale the volume.
    pub fn change_volume(&self, out: &str, scale: f64) -> io::Result<()> {
        if WavAudioFile16::is_wav_filename(out) {
            let mut outfile: File = File::create(out)?;
            outfile.write(&self.header.to_bytes())?;

            for sample in self.data.iter() {
                let scaled = (((*sample as f64) * scale) as i16);
                outfile.write(&scaled.to_le_bytes())?;
            }

            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "File should be a wav file"))
        }
    }

    /// Checks if the file's name belongs to a WAV file.
    ///
    /// # Arguments
    /// * `filename` - Filename to check.
    pub fn is_wav_filename(filename: &str) -> bool {
        match filename.split('.').last() {
            Some("wav") => true,
            _ => false
        }
    }
}

pub fn main() {
    // Reads input file, output file and volume scale from command line args.
    let args: Vec<String> = env::args().collect();

    let (input, output, scale): (&str, &str, f64) = match &args[1..] {
        [i, o, s] => (i, o, s.parse().expect("Scale should be a number")),
        _ => panic!("Usage:\n./volume <input> <output> <scale factor>")
    };

    // Reads the WAV file
    let file = match WavAudioFile16::new(input) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };

    // Changes the file's volume and writes the scaled samples to the output file.
    match file.change_volume(output, scale) {
        Err(e) => panic!("{:?}", e),
        _ => ()
    };
}
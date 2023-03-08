use std::fs::File;
use std::{io, mem, num};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::{Add, Mul, RangeInclusive};
use num_traits::PrimInt;

use super::helpers;

pub const BMP_HEADER_SIZE: usize= 14;
pub const BMP_INFO_HEADER_SIZE: usize = 40;

const KERNEL: [[i32; 3]; 3] = [
    [-1, -2, -1],
    [0, 0, 0],
    [1, 2, 1]
];

#[derive(Debug, Copy, Clone)]
pub struct Color<T: PrimInt>(T, T, T);

impl Color<u8> {
    pub fn to_be_bytes(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }

    pub fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(bytes[0], bytes[1], bytes[2])
    }

    pub fn to_le_bytes(&self) -> [u8; 3] {
        [self.2, self.1, self.0]
    }

    pub fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self(bytes[2], bytes[1], bytes[0])
    }
}

impl <T: PrimInt> Add for Color<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl <T: PrimInt> Mul for Color<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl <T: PrimInt> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

pub enum ImageFilter {
    GrayScale,
    Sepia,
    Reflection,
    Blur,
    Edges
}

pub struct BMPFileHeader {
    pub bf_type: u16,
    pub bf_size: u32,
    pub bf_reserved1: u16,
    pub bf_reserved2: u16,
    pub bf_off_bits: u32
}

impl BMPFileHeader {
    pub fn new(bytes: &[u8]) -> Self {
        BMPFileHeader {
            bf_type: u16::from_le_bytes(helpers::slice2(&bytes[0..2])),
            bf_size: u32::from_le_bytes(helpers::slice4(&bytes[2..6])),
            bf_reserved1: u16::from_le_bytes(helpers::slice2(&bytes[6..8])),
            bf_reserved2: u16::from_le_bytes(helpers::slice2(&bytes[8..10])),
            bf_off_bits: u32::from_le_bytes(helpers::slice4(&bytes[10..14]))
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.bf_type.to_le_bytes());
        bytes.extend_from_slice(&self.bf_size.to_le_bytes());
        bytes.extend_from_slice(&self.bf_reserved1.to_le_bytes());
        bytes.extend_from_slice(&self.bf_reserved2.to_le_bytes());
        bytes.extend_from_slice(&self.bf_off_bits.to_le_bytes());
        bytes
    }
}

pub struct BMPInfoHeader {
    pub bi_size: u32,
    pub bi_width: i32,
    pub bi_height: i32,
    pub bi_planes: u16,
    pub bi_bit_count: u16,
    pub bi_compression: u32,
    pub bi_image_size: u32,
    pub bi_resolution_x: i32,
    pub bi_resolution_y: i32,
    pub bi_colors: u32,
    pub bi_colors_important: u32
}

impl BMPInfoHeader {
    pub fn new(bytes: &[u8]) -> Self {
        BMPInfoHeader {
            bi_size: u32::from_le_bytes(helpers::slice4(&bytes[0..4])),
            bi_width: i32::from_le_bytes(helpers::slice4(&bytes[4..8])),
            bi_height: i32::from_le_bytes(helpers::slice4(&bytes[8..12])),
            bi_planes: u16::from_le_bytes(helpers::slice2(&bytes[12..14])),
            bi_bit_count: u16::from_le_bytes(helpers::slice2(&bytes[14..16])),
            bi_compression: u32::from_le_bytes(helpers::slice4(&bytes[16..20])),
            bi_image_size: u32::from_le_bytes(helpers::slice4(&bytes[20..24])),
            bi_resolution_x: i32::from_le_bytes(helpers::slice4(&bytes[24..28])),
            bi_resolution_y: i32::from_le_bytes(helpers::slice4(&bytes[28..32])),
            bi_colors: u32::from_le_bytes(helpers::slice4(&bytes[32..36])),
            bi_colors_important: u32::from_le_bytes(helpers::slice4(&bytes[36..40])),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.bi_size.to_le_bytes());
        bytes.extend_from_slice(&self.bi_width.to_le_bytes());
        bytes.extend_from_slice(&self.bi_height.to_le_bytes());
        bytes.extend_from_slice(&self.bi_planes.to_le_bytes());
        bytes.extend_from_slice(&self.bi_bit_count.to_le_bytes());
        bytes.extend_from_slice(&self.bi_compression.to_le_bytes());
        bytes.extend_from_slice(&self.bi_image_size.to_le_bytes());
        bytes.extend_from_slice(&self.bi_resolution_x.to_le_bytes());
        bytes.extend_from_slice(&self.bi_resolution_y.to_le_bytes());
        bytes.extend_from_slice(&self.bi_colors.to_le_bytes());
        bytes.extend_from_slice(&self.bi_colors_important.to_le_bytes());
        bytes
    }
}

pub struct BMPFile24 {
    pub bf_header: BMPFileHeader,
    pub bi_header: BMPInfoHeader,
    pub data: Vec<Vec<Color<u8>>>
}

impl BMPFile24 {
    pub fn new(filename: &str) -> io::Result<Self> {
        if BMPFile24::is_bmp_filename(filename) {
            let mut file = File::open(filename)?;
            let mut reader = BufReader::with_capacity( 65536, file);

            let mut bf_buffer: [u8; BMP_HEADER_SIZE] = [0; BMP_HEADER_SIZE];
            reader.read_exact(&mut bf_buffer)?;
            let bf_header = BMPFileHeader::new(&bf_buffer);

            let mut bi_buffer: [u8; BMP_INFO_HEADER_SIZE] = [0; BMP_INFO_HEADER_SIZE];
            reader.read_exact(&mut bi_buffer)?;
            let bi_header = BMPInfoHeader::new(&bi_buffer);

            if bf_header.bf_type != 0x4d42 || bf_header.bf_off_bits != 54 || bi_header.bi_size != 40 ||
                bi_header.bi_bit_count != 24 || bi_header.bi_compression != 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "The file is not a 24 bit BMP file"));
            }

            let (width, height) = (bi_header.bi_width as usize, bi_header.bi_height.abs() as usize);
            let padding = ((4 - (width * mem::size_of::<Color<u8>>()) % 4) % 4) as i64;
            let mut data = Vec::with_capacity(height);
            let mut data_buffer: Vec<u8> = vec![0; width * mem::size_of::<Color<u8>>()];

            for _ in 0..height {
                reader.read_exact(&mut data_buffer)?;
                let mut row: Vec<Color<u8>> = Vec::with_capacity(width);

                for i in 0..width {
                    row.push(Color::<u8>::from_le_bytes([data_buffer[3 * i], data_buffer[3 * i + 1], data_buffer[3 * i + 2]]));
                }

                data.push(row);
                reader.seek(SeekFrom::Current(padding))?;
            }

            Ok(BMPFile24 {
                bf_header,
                bi_header,
                data
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "File should be a wav file"))
        }
    }

    pub fn transform<F: Fn(&Vec<Vec<Color<u8>>>, usize, usize) -> Color<u8>>(&self, out: &str, transform: F) -> io::Result<()> {
        if Self::is_bmp_filename(out) {
            let mut outfile: File = File::create(out)?;
            let width = self.bi_header.bi_width as usize;
            let mut writer = BufWriter::with_capacity(65536, outfile);
            writer.write(&self.bf_header.to_bytes())?;
            writer.write(&self.bi_header.to_bytes())?;

            let padding = ((4 - (width * mem::size_of::<Color<u8>>()) % 4) % 4) as i64;
            let mut bytes: Vec<u8> = Vec::with_capacity(self.bf_header.bf_size as usize);

            for i in 0..self.data.len() {
                for j in 0..self.data[i].len() {
                    let new_pixel = transform(&self.data, i, j);
                    bytes.extend_from_slice(&new_pixel.to_le_bytes());
                }

                bytes.resize(bytes.len() + padding as usize, 0);
            }

            writer.write(&bytes)?;

            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "File should be a BMP file"))
        }
    }

    pub fn copy(&self, out: &str) -> io::Result<()> {
        self.transform(out, |image: &Vec<Vec<Color<u8>>>, i, j| image[i][j].clone())
    }

    pub fn filter(&self, out: &str, filter_type: ImageFilter) -> io::Result<()> {
        let action: fn(&Vec<Vec<Color<u8>>>, usize, usize) -> Color<u8> = match filter_type {
            ImageFilter::GrayScale => |image: &Vec<Vec<Color<u8>>>, i, j| {
                let row: &Vec<Color<u8>> = &image[i];
                let pixel: &Color<u8> = &row[j];
                let gray = ((pixel.0 as u32 + pixel.1 as u32 + pixel.2 as u32) / 3) as u8;
                Color(gray, gray, gray)
            },
            ImageFilter::Sepia => |image: &Vec<Vec<Color<u8>>>, i, j| {
                let row: &Vec<Color<u8>> = &image[i];
                let pixel: [u8; 3] = row[j].to_be_bytes();

                let sepia = |constants: &[f64; 3]| pixel
                    .into_iter()
                    .zip(constants.into_iter())
                    .map(|(b, c)| c * b as f64)
                    .sum::<f64>()
                    .round()
                    .clamp(0.0, 255.0) as u8;

                let new_color = [
                    &[0.393, 0.769, 0.189],
                    &[0.349, 0.686, 0.168],
                    &[0.272, 0.534, 0.131],
                ].map(sepia);

                Color::<u8>::from_be_bytes(new_color)
            },
            ImageFilter::Reflection => |image: &Vec<Vec<Color<u8>>>, i, j| -> Color<u8> {
                let row: &Vec<Color<u8>> = &image[i];
                row[row.len() - j - 1].clone()
            },
            ImageFilter::Blur => |image: &Vec<Vec<Color<u8>>>, i, j| {
                let (height, width) = (image.len(), image[i].len());

                let (r, g, b, n): (u32, u32, u32, u32) = adjacent_range(i, 3, 0..=height - 1)
                    .fold((0, 0, 0, 0), |sum, y| {
                        let row: &Vec<Color<u8>> = &image[y];

                        adjacent_range(j, 3, 0..=width - 1)
                            .fold(sum, |(r, g, b, n), x| {
                                let pixel = &row[x];
                                (r + pixel.0 as u32, g + pixel.1 as u32, b + pixel.2 as u32, n + 1)
                            })
                    });

                Color((r / n) as u8, (g / n) as u8, (b / n) as u8)
            },
            ImageFilter::Edges => |image: &Vec<Vec<Color<u8>>>, i, j| {
                let (height, width) = (image.len(), image[i].len());

                let (cx, cy): (Color<i32>, Color<i32>) = adjacent_range(i, 1, 0..=height - 1)
                    .enumerate()
                    .fold((Color(0, 0, 0), Color(0, 0, 0)), |(gx, gy), (i2, y)| {
                        let row: &Vec<Color<u8>> = &image[y];
                        let offset_x = if j == 0 { 1 } else { 0 };
                        let offset_y = if i == 0 { 1 } else { 0 };

                        adjacent_range(j, 1, 0..=width - 1)
                            .enumerate()
                            .fold((gx, gy), |(mut gx2, mut gy2), (j2, x)| {
                                let pixel = Color(row[x].0 as i32, row[x].1 as i32, row[x].2 as i32);
                                let (k_x, k_y) = (KERNEL[offset_x + j2][offset_y + i2], KERNEL[offset_y + i2][offset_x + j2]);

                                gx2 = gx2 + pixel * k_x;
                                gy2 = gy2 + pixel * k_y;
                                (gx2, gy2)
                            })
                    });

                let r = ((cx.0.pow(2) + cy.0.pow(2)) as f64).sqrt().round().clamp(0.0, 255.0) as u8;
                let g = ((cx.1.pow(2) + cy.1.pow(2)) as f64).sqrt().round().clamp(0.0, 255.0) as u8;
                let b = ((cx.2.pow(2) + cy.2.pow(2)) as f64).sqrt().round().clamp(0.0, 255.0) as u8;

                Color(r, g, b)
            }
        };

        self.transform(out, action)
    }

    pub fn is_bmp_filename(filename: &str) -> bool {
        match filename.split('.').last() {
            Some("bmp") => true,
            _ => false
        }
    }
}

fn adjacent_range(idx: usize, diff: usize,  range: RangeInclusive<usize>) -> RangeInclusive<usize> {
    let (&start, &end) = (range.start(), range.end());

    match idx {
        idx if idx - start < diff => start..=idx + diff,
        idx if end - idx < diff => idx - diff..=end,
        _ => idx - diff..=idx + diff
    }
}
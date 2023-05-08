use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
use chrono::{Duration, NaiveDate};
use csv::ReaderBuilder;
use reqwest;

/// The dataset's URL
const URL: &str = "https://raw.githubusercontent.com/nytimes/covid-19-data/master/us-states.csv";

/// A state's daily covid record which was deserialized from a CSV file.
#[derive(Deserialize, Debug)]
struct CsvCovidRecord {
    // The state where the covid cases and deaths happened.
    state: String,
    // The state's fips.
    fips: u8,
    // Number of cumulative cases.
    cases: i32,
    // Number of cumulative deaths.
    deaths: i32,
    // The date where the cases happened.
    #[serde(deserialize_with = "deserialize_from_str")]
    date: NaiveDate
}

/// A daily covid record without state data.
#[derive(Debug)]
struct CovidRecord {
    // Number of cumulative cases.
    cases: i32,
    // Number of cumulative deaths.
    deaths: i32,
    // The date where the cases happened.
    date: NaiveDate
}

impl CovidRecord {
    /// Creates a new covid record with the given data.
    ///
    /// # Arguments
    /// * `cases` - Number of cumulative cases.
    /// * `deaths` - Number of cumulative deaths.
    /// * `date` - The date where the cases happened.
    pub fn new(cases: i32, deaths: i32, date: NaiveDate) -> Self {
        Self { cases, deaths, date }
    }

    /// Takes a list of covid records each belonging to a different state and associates each record with it's state.
    /// Returns a hashmap where the state is the key and the record is the value.
    ///
    /// # Arguments
    /// * `records` - List of covid records to associate.
    pub fn associate(records: Vec<CsvCovidRecord>) -> HashMap<String, Self> {
        records.into_iter()
            .map(|record| (record.state, Self::new(record.cases, record.deaths, record.date)))
            .collect()
    }

    /// Takes a list of covid records and groups them by state.
    /// Returns a hashmap where the state is the key and the state's records are the value.
    ///
    /// # Arguments
    /// * `records` - List of covid records to group by state.
    pub fn group(records: Vec<CsvCovidRecord>) -> HashMap<String, Vec<Self>> {
        let mut groups = HashMap::new();

        for record in records {
            let new_record = Self::new(record.cases, record.deaths, record.date);
            groups.entry(record.state).or_insert_with(|| Vec::new()).push(new_record);
        }

        groups
    }

    /// Takes a list of covid records and computes their average daily cases.
    ///
    /// # Arguments
    /// * `records` - The list of covid records.
    pub fn average(records: &[Self]) -> i32 {
        let len = records.len();

        records.into_iter()
            .map(|record| record.cases)
            .sum::<i32>() / len as i32
    }
}

/// A hashmap which maps a state to it's covid records.
type StateRecords = HashMap<String, Vec<CovidRecord>>;

// Deserializes data implementing the FromStr trait.
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
    where
        S: FromStr,      // Required for S::from_str...
        S::Err: Display, // Required for .map_err(de::Error::custom)
        D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&string).map_err(de::Error::custom)
}

/// Takes a list of csv covid records and groups them by state, then calculates the daily cases for each record.
/// Returns the records grouped by state with the correct amount of cases and deaths.
///
/// # Arguments
/// * `records` - The list of csv covid records.
fn calculate(records: Vec<CsvCovidRecord>) -> StateRecords {
    let max_date = records.last().unwrap().date;

    let (base_cases, mut records): (Vec<_>, Vec<_>) = records
        .into_iter()
        .rev()
        .take_while(|record| (max_date - record.date) <= Duration::days(15))
        .partition(|record| (max_date - record.date) == Duration::days(15));

    records.reverse();
    let base_data = CovidRecord::associate(base_cases);
    let mut state_records = CovidRecord::group(records);

    for (state, records) in state_records.iter_mut() {
        let mut base_cases = base_data[state].cases;
        let mut base_deaths = base_data[state].deaths;

        for i in 0..records.len() {
            records[i].cases -= base_cases;
            records[i].deaths -= base_deaths;
            base_cases += records[i].cases;
            base_deaths += records[i].deaths;
        }
    }

    state_records
}

/// Takes a hashmap which maps each state to it's records and then calculates the average daily cases for the last 2 weeks for each state.
/// Returns a hashmap where each state is the key and the value is a tuple containing the average daily cases of the last week and the percent change compared to the week before that.
///
/// # Arguments
/// * `state_records` - A hashmap which maps each state to it's records.
fn comparative_averages(state_records: StateRecords) -> HashMap<String, (i32, i32)> {
    state_records.into_iter()
        .map(|(state, record)| {
            let week_avg = CovidRecord::average(&record[..7]);
            let last_week_avg = CovidRecord::average(&record[7..]);
            let percent = if last_week_avg == 0 { 100 } else { 100 * (week_avg - last_week_avg) / last_week_avg };

            (state, (week_avg, percent))
        })
        .collect()
}

pub fn main() {
    // Downloads dataset and reads as CSV.
    let response = reqwest::blocking::get(URL).unwrap();
    let mut reader = ReaderBuilder::new().from_reader(response);
    let records = reader.deserialize().collect::<Result<Vec<CsvCovidRecord>, _>>().unwrap();

    // Groups the records by state and calculates daily cases and deaths.
    let state_records = calculate(records);

    // Show the daily average cases for each state and the percent change.
    for (state, (average, percent)) in comparative_averages(state_records) {
        println!("{state} had a 7-day average of {average} and a {} of {}%.", if percent < 0 { "decrease" } else { "increase" }, percent.abs())
    }
}
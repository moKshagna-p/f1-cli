use serde::{Deserialize, Serialize};
use anyhow::Result;

const API_BASE: &str = "https://api.openf1.org/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_key: i32,
    pub circuit_short_name: String,
    pub location: String,
    pub session_type: String,
    pub date_start: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub driver_number: i32,
    pub name_acronym: String,
    pub first_name: String,
    pub last_name: String,
    pub team_name: String,
    pub team_colour: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub driver_number: i32,
    pub position: Option<i32>,
    pub gap_to_leader: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lap {
    pub driver_number: i32,
    pub lap_number: i32,
    pub lap_duration: Option<f64>,
    pub sector1_duration: Option<f64>,
    pub sector2_duration: Option<f64>,
    pub sector3_duration: Option<f64>,
    pub is_pit_out_lap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitStop {
    pub driver_number: i32,
    pub stop_number: i32,
    pub pit_duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub air_temperature: Option<f64>,
    pub track_temperature: Option<f64>,
    pub rainfall: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub drivers: Vec<Driver>,
    pub positions: Vec<Position>,
    pub laps: Vec<Lap>,
    pub pit_stops: Vec<PitStop>,
    pub weather: Vec<Weather>,
}

pub async fn fetch_sessions() -> Result<Session> {
    let client = reqwest::Client::new();
    let url = format!("{}/sessions", API_BASE);
    
    let sessions: Vec<Session> = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?
        .json()
        .await?;

    Ok(sessions
        .into_iter()
        .max_by_key(|s| s.date_start.clone())
        .unwrap_or_else(|| Session {
            session_key: 0,
            circuit_short_name: "Unknown".to_string(),
            location: "Unknown".to_string(),
            session_type: "Practice".to_string(),
            date_start: String::new(),
        }))
}

pub async fn fetch_drivers() -> Result<Vec<Driver>> {
    let client = reqwest::Client::new();
    let url = format!("{}/drivers", API_BASE);
    
    let drivers: Vec<Driver> = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?
        .json()
        .await?;

    Ok(drivers)
}

pub async fn fetch_all_telemetry(session_key: &str) -> Result<Telemetry> {
    let client = reqwest::Client::new();

    let (drivers, positions, laps, pit_stops, weather) = tokio::join!(
        fetch_drivers_internal(&client),
        fetch_positions(&client, session_key),
        fetch_laps(&client, session_key),
        fetch_pit_stops(&client, session_key),
        fetch_weather(&client, session_key),
    );

    Ok(Telemetry {
        drivers: drivers.unwrap_or_default(),
        positions: positions.unwrap_or_default(),
        laps: laps.unwrap_or_default(),
        pit_stops: pit_stops.unwrap_or_default(),
        weather: weather.unwrap_or_default(),
    })
}

async fn fetch_drivers_internal(client: &reqwest::Client) -> Result<Vec<Driver>> {
    let url = format!("{}/drivers", API_BASE);
    Ok(client.get(&url).send().await?.json().await?)
}

async fn fetch_positions(client: &reqwest::Client, session_key: &str) -> Result<Vec<Position>> {
    let url = format!("{}/position?session_key={}", API_BASE, session_key);
    Ok(client.get(&url).send().await?.json().await?)
}

async fn fetch_laps(client: &reqwest::Client, session_key: &str) -> Result<Vec<Lap>> {
    let url = format!("{}/laps?session_key={}&limit=1000", API_BASE, session_key);
    Ok(client.get(&url).send().await?.json().await?)
}

async fn fetch_pit_stops(client: &reqwest::Client, session_key: &str) -> Result<Vec<PitStop>> {
    let url = format!("{}/pit?session_key={}", API_BASE, session_key);
    Ok(client.get(&url).send().await?.json().await?)
}

async fn fetch_weather(client: &reqwest::Client, session_key: &str) -> Result<Vec<Weather>> {
    let url = format!("{}/weather?session_key={}", API_BASE, session_key);
    Ok(client.get(&url).send().await?.json().await?)
}

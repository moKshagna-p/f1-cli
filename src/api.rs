use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use std::sync::Arc;

const API_BASE: &str = "https://api.openf1.org/v1";

// ─── API Response Models ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub session_key: i64,
    #[serde(default)]
    pub session_name: String,
    #[serde(default)]
    pub session_type: String,
    #[serde(default)]
    pub circuit_short_name: String,
    #[serde(default)]
    pub circuit_key: i64,
    #[serde(default)]
    pub country_name: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub date_start: String,
    #[serde(default)]
    pub date_end: String,
    #[serde(default)]
    pub year: i32,
    #[serde(default)]
    pub meeting_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub driver_number: i32,
    #[serde(default)]
    pub name_acronym: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub team_name: String,
    #[serde(default)]
    pub team_colour: Option<String>,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub headshot_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub driver_number: i32,
    pub position: Option<i32>,
    #[serde(default)]
    pub gap_to_leader: Option<f64>,
    #[serde(default)]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lap {
    pub driver_number: i32,
    pub lap_number: i32,
    #[serde(default)]
    pub lap_duration: Option<f64>,
    #[serde(default)]
    pub duration_sector_1: Option<f64>,
    #[serde(default)]
    pub duration_sector_2: Option<f64>,
    #[serde(default)]
    pub duration_sector_3: Option<f64>,
    #[serde(default)]
    pub i1_speed: Option<f64>,
    #[serde(default)]
    pub i2_speed: Option<f64>,
    #[serde(default)]
    pub st_speed: Option<f64>,
    #[serde(default)]
    pub is_pit_out_lap: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitStop {
    pub driver_number: i32,
    pub stop_number: i32,
    #[serde(default)]
    pub pit_duration: Option<f64>,
    #[serde(default)]
    pub lap_number: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Weather {
    #[serde(default)]
    pub air_temperature: Option<f64>,
    #[serde(default)]
    pub track_temperature: Option<f64>,
    #[serde(default)]
    pub humidity: Option<f64>,
    #[serde(default)]
    pub pressure: Option<f64>,
    #[serde(default)]
    pub wind_direction: Option<i32>,
    #[serde(default)]
    pub wind_speed: Option<f64>,
    #[serde(default)]
    pub rainfall: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interval {
    pub driver_number: i32,
    #[serde(default)]
    pub gap_to_leader: Option<f64>,
    #[serde(default)]
    pub interval: Option<f64>,
    #[serde(default)]
    pub date: Option<String>,
}

// ─── Aggregated Telemetry Bundle ──────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct Telemetry {
    pub drivers: Vec<Driver>,
    pub positions: Vec<Position>,
    pub laps: Vec<Lap>,
    pub pit_stops: Vec<PitStop>,
    pub weather: Weather,
    pub intervals: Vec<Interval>,
}

// ─── Shared HTTP Client ───────────────────────────────────────────────────────

pub fn make_client() -> Arc<reqwest::Client> {
    Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .user_agent("f1-dashboard/2.0")
            .build()
            .expect("Failed to build HTTP client"),
    )
}

// ─── Safe JSON Fetching ───────────────────────────────────────────────────────

/// Fetch URL and deserialize as Vec<T>.
/// Gracefully handles API error objects like `{"detail":"No results found."}`.
async fn fetch_array<T>(client: &reqwest::Client, url: &str) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let resp = client.get(url).send().await?;
    let bytes = resp.bytes().await?;

    // Happy path: parse as JSON array
    if let Ok(arr) = serde_json::from_slice::<Vec<T>>(&bytes) {
        return Ok(arr);
    }

    // Capture raw body for diagnostics
    let raw: String = String::from_utf8_lossy(&bytes).chars().take(300).collect();

    // Try to extract an error message from a JSON object response
    if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        let msg = val.get("detail")
            .or_else(|| val.get("error"))
            .or_else(|| val.get("message"))
            .and_then(|v| v.as_str());

        if let Some(text) = msg {
            // "No results" is a legitimate empty response
            if text.to_lowercase().contains("no results")
                || text.to_lowercase().contains("not found")
            {
                return Ok(vec![]);
            }
            return Err(anyhow!("API error: {}", text));
        }

        // Empty JSON object {} = no data
        if val.as_object().map(|o| o.is_empty()).unwrap_or(false) {
            return Ok(vec![]);
        }
    }

    Err(anyhow!("Unexpected API response\nBody: {}", raw))
}

// ─── API Functions ────────────────────────────────────────────────────────────

/// Returns the most recent session that has already started.
/// Filters out future scheduled sessions that have no live data.
pub async fn fetch_latest_session(client: &reqwest::Client) -> Result<Session> {
    let url = format!("{}/sessions", API_BASE);
    let mut sessions: Vec<Session> = fetch_array(client, &url).await?;

    if sessions.is_empty() {
        return Err(anyhow!("No sessions returned from OpenF1 API"));
    }

    // Sort ascending by date_start (ISO8601 strings sort lexicographically)
    sessions.sort_by(|a, b| a.date_start.cmp(&b.date_start));

    let now = chrono::Utc::now();

    // Walk from newest to oldest, pick first one that's already started
    let best = sessions.iter().rev().find(|s| {
        // Parse RFC3339 date — API format: "2026-05-22T20:30:00+00:00"
        chrono::DateTime::parse_from_rfc3339(&s.date_start)
            .map(|dt| dt <= now)
            .unwrap_or(false)
    });

    match best {
        Some(s) => Ok(s.clone()),
        // Extremely unlikely: all sessions are in the future
        None => Ok(sessions.into_iter().last().unwrap()),
    }
}

pub async fn fetch_drivers_for_session(
    client: &reqwest::Client,
    session_key: i64,
) -> Result<Vec<Driver>> {
    let url = format!("{}/drivers?session_key={}", API_BASE, session_key);
    fetch_array(client, &url).await
}

pub async fn fetch_all_telemetry(client: &reqwest::Client, session_key: i64) -> Result<Telemetry> {
    let (positions_r, laps_r, pits_r, weather_r, intervals_r) = tokio::join!(
        fetch_positions(client, session_key),
        fetch_laps(client, session_key),
        fetch_pit_stops(client, session_key),
        fetch_weather(client, session_key),
        fetch_intervals(client, session_key),
    );

    Ok(Telemetry {
        drivers: vec![],
        positions: positions_r.unwrap_or_default(),
        laps: laps_r.unwrap_or_default(),
        pit_stops: pits_r.unwrap_or_default(),
        weather: weather_r.unwrap_or_default(),
        intervals: intervals_r.unwrap_or_default(),
    })
}

async fn fetch_positions(client: &reqwest::Client, session_key: i64) -> Result<Vec<Position>> {
    let url = format!("{}/position?session_key={}", API_BASE, session_key);
    fetch_array(client, &url).await
}

async fn fetch_laps(client: &reqwest::Client, session_key: i64) -> Result<Vec<Lap>> {
    let url = format!("{}/laps?session_key={}", API_BASE, session_key);
    fetch_array(client, &url).await
}

async fn fetch_pit_stops(client: &reqwest::Client, session_key: i64) -> Result<Vec<PitStop>> {
    let url = format!("{}/pit?session_key={}", API_BASE, session_key);
    fetch_array(client, &url).await
}

async fn fetch_weather(client: &reqwest::Client, session_key: i64) -> Result<Weather> {
    let url = format!("{}/weather?session_key={}", API_BASE, session_key);
    let readings: Vec<Weather> = fetch_array(client, &url).await?;
    Ok(readings.into_iter().last().unwrap_or_default())
}

async fn fetch_intervals(client: &reqwest::Client, session_key: i64) -> Result<Vec<Interval>> {
    let url = format!("{}/intervals?session_key={}", API_BASE, session_key);
    fetch_array(client, &url).await
}

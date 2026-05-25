use crate::api::{Driver, Session, Telemetry, Weather};
use std::collections::HashMap;
use std::time::Instant;

fn format_gap(val: &serde_json::Value) -> Option<String> {
    match val {
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f <= 0.001 {
                    None
                } else {
                    Some(format!("+{:.3}", f))
                }
            } else {
                Some(n.to_string())
            }
        }
        serde_json::Value::String(s) => {
            if s.is_empty() || s == "0.0" {
                None
            } else {
                Some(s.clone())
            }
        }
        _ => None,
    }
}

// ─── Driver Standing Record ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DriverStanding {
    pub number: i32,
    pub acronym: String,
    pub team: String,
    pub team_color: String,
    pub country: String,
    pub position: i32,
    /// Gap to the leader in seconds or text (None = leader)
    pub gap_to_leader: Option<String>,
    /// Gap to the car directly ahead
    pub interval: Option<String>,
    pub lap_number: i32,
    pub lap_time: Option<f64>,
    pub sector1: Option<f64>,
    pub sector2: Option<f64>,
    pub sector3: Option<f64>,
    pub pit_stops: i32,
    pub last_pit_lap: Option<i32>,
    pub is_fastest_lap: bool,
    pub fastest_lap: Option<f64>,
    /// 1 = gained, -1 = lost, 0 = unchanged
    pub position_delta: i32,
    pub delta_fade: u8, // ticks remaining for animation (0 = done)
}

#[derive(Debug, Clone)]
pub struct ChampionshipStanding {
    pub position: String,
    pub points: String,
    pub wins: String,
    pub driver_name: String,
    pub team_color: String,
}

impl DriverStanding {
    pub fn gap_display(&self) -> String {
        match &self.gap_to_leader {
            None => "LEADER".to_string(),
            Some(s) => s.clone(),
        }
    }

    pub fn interval_display(&self) -> String {
        match &self.interval {
            None => "—".to_string(),
            Some(s) => s.clone(),
        }
    }
}

// ─── App State ────────────────────────────────────────────────────────────────

pub struct AppState {
    pub session: Session,
    pub standings: Vec<DriverStanding>,
    pub championship: Vec<ChampionshipStanding>,
    pub weather: Weather,
    pub last_updated: Instant,
    pub error: Option<String>,

    // Internal tracking for diffs
    last_positions: HashMap<i32, i32>,
    fastest_lap: Option<(i32, f64)>, // (driver_number, time)
    fastest_s1: Option<(i32, f64)>,
    fastest_s2: Option<(i32, f64)>,
    fastest_s3: Option<(i32, f64)>,
    tick: u64,
}

impl AppState {
    pub fn new(session: Session, drivers: Vec<Driver>) -> Self {
        let standings = drivers
            .iter()
            .enumerate()
            .map(|(i, d)| DriverStanding {
                number: d.driver_number,
                acronym: d.name_acronym.clone(),
                team: d.team_name.clone(),
                team_color: d
                    .team_colour
                    .clone()
                    .unwrap_or_else(|| "FFFFFF".to_string()),
                country: d.country_code.clone(),
                position: (i + 1) as i32,
                gap_to_leader: if i == 0 { None } else { Some(0.0.to_string()) },
                interval: None,
                lap_number: 0,
                lap_time: None,
                sector1: None,
                sector2: None,
                sector3: None,
                pit_stops: 0,
                last_pit_lap: None,
                is_fastest_lap: false,
                fastest_lap: None,
                position_delta: 0,
                delta_fade: 0,
            })
            .collect();

        let last_positions = drivers
            .iter()
            .enumerate()
            .map(|(i, d)| (d.driver_number, (i + 1) as i32))
            .collect();

        Self {
            session,
            standings,
            championship: Vec::new(),
            weather: Weather::default(),
            last_updated: Instant::now(),
            error: None,
            last_positions,
            fastest_lap: None,
            fastest_s1: None,
            fastest_s2: None,
            fastest_s3: None,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        // Fade position delta animations after 20 ticks (~4s at 200ms)
        for s in &mut self.standings {
            if s.delta_fade > 0 {
                s.delta_fade -= 1;
                if s.delta_fade == 0 {
                    s.position_delta = 0;
                }
            }
        }
    }

    pub fn update(&mut self, telemetry: Telemetry) {
        self.weather = telemetry.weather;
        self.error = None;
        self.last_updated = Instant::now();

        // ── Positions ─────────────────────────────────────────────────────────
        // Collect latest position per driver (last entry wins)
        let mut latest_pos: HashMap<i32, i32> = HashMap::new();
        let mut latest_gap: HashMap<i32, String> = HashMap::new();
        for pos in &telemetry.positions {
            if let Some(p) = pos.position {
                latest_pos.insert(pos.driver_number, p);
            }
            if let Some(g) = &pos.gap_to_leader {
                if let Some(s) = format_gap(g) {
                    latest_gap.insert(pos.driver_number, s);
                }
            }
        }

        // ── Intervals ─────────────────────────────────────────────────────────
        let mut latest_interval: HashMap<i32, String> = HashMap::new();
        let mut latest_interval_gap: HashMap<i32, String> = HashMap::new();
        for iv in &telemetry.intervals {
            if let Some(g) = &iv.gap_to_leader {
                if let Some(s) = format_gap(g) {
                    latest_interval_gap.insert(iv.driver_number, s);
                }
            }
            if let Some(i) = &iv.interval {
                if let Some(s) = format_gap(i) {
                    latest_interval.insert(iv.driver_number, s);
                }
            }
        }

        // ── Laps ──────────────────────────────────────────────────────────────
        let mut latest_laps: HashMap<i32, &crate::api::Lap> = HashMap::new();
        for lap in &telemetry.laps {
            latest_laps
                .entry(lap.driver_number)
                .and_modify(|e| {
                    if lap.lap_number > e.lap_number {
                        *e = lap;
                    }
                })
                .or_insert(lap);
        }

        // ── Pit Stops ─────────────────────────────────────────────────────────
        let mut max_pit: HashMap<i32, (i32, Option<i32>)> = HashMap::new(); // (stop_num, lap)
        for pit in &telemetry.pit_stops {
            max_pit
                .entry(pit.driver_number)
                .and_modify(|e| {
                    if pit.stop_number > e.0 {
                        *e = (pit.stop_number, pit.lap_number);
                    }
                })
                .or_insert((pit.stop_number, pit.lap_number));
        }

        // ── Fastest lap tracking ───────────────────────────────────────────────
        let mut new_fastest_lap: Option<(i32, f64)> = None;
        let mut new_fastest_s1: Option<(i32, f64)> = None;
        let mut new_fastest_s2: Option<(i32, f64)> = None;
        let mut new_fastest_s3: Option<(i32, f64)> = None;

        for (&dn, &lap) in &latest_laps {
            if let Some(t) = lap.lap_duration {
                match new_fastest_lap {
                    None => new_fastest_lap = Some((dn, t)),
                    Some((_, ft)) if t < ft => new_fastest_lap = Some((dn, t)),
                    _ => {}
                }
            }
            if let Some(s) = lap.duration_sector_1 {
                match new_fastest_s1 {
                    None => new_fastest_s1 = Some((dn, s)),
                    Some((_, fs)) if s < fs => new_fastest_s1 = Some((dn, s)),
                    _ => {}
                }
            }
            if let Some(s) = lap.duration_sector_2 {
                match new_fastest_s2 {
                    None => new_fastest_s2 = Some((dn, s)),
                    Some((_, fs)) if s < fs => new_fastest_s2 = Some((dn, s)),
                    _ => {}
                }
            }
            if let Some(s) = lap.duration_sector_3 {
                match new_fastest_s3 {
                    None => new_fastest_s3 = Some((dn, s)),
                    Some((_, fs)) if s < fs => new_fastest_s3 = Some((dn, s)),
                    _ => {}
                }
            }
        }

        if new_fastest_lap.is_some() {
            self.fastest_lap = new_fastest_lap;
        }
        if new_fastest_s1.is_some() {
            self.fastest_s1 = new_fastest_s1;
        }
        if new_fastest_s2.is_some() {
            self.fastest_s2 = new_fastest_s2;
        }
        if new_fastest_s3.is_some() {
            self.fastest_s3 = new_fastest_s3;
        }

        // ── Update standings ──────────────────────────────────────────────────
        for standing in &mut self.standings {
            let dn = standing.number;

            // Position + delta
            if let Some(&new_pos) = latest_pos.get(&dn) {
                let old_pos = *self.last_positions.get(&dn).unwrap_or(&new_pos);
                if new_pos != old_pos {
                    standing.position_delta = if new_pos < old_pos { 1 } else { -1 };
                    standing.delta_fade = 20;
                }
                standing.position = new_pos;
                self.last_positions.insert(dn, new_pos);
            }

            // Gap — prefer intervals endpoint, fall back to positions
            if let Some(gap) = latest_interval_gap.get(&dn) {
                standing.gap_to_leader = Some(gap.clone());
            } else if let Some(gap) = latest_gap.get(&dn) {
                standing.gap_to_leader = Some(gap.clone());
            }

            // Interval to car ahead
            if let Some(iv) = latest_interval.get(&dn) {
                standing.interval = Some(iv.clone());
            }

            // Lap data
            if let Some(&lap) = latest_laps.get(&dn) {
                standing.lap_number = lap.lap_number;
                standing.lap_time = lap.lap_duration;
                standing.sector1 = lap.duration_sector_1;
                standing.sector2 = lap.duration_sector_2;
                standing.sector3 = lap.duration_sector_3;
            }

            // Pit stops
            if let Some(&(stop_num, stop_lap)) = max_pit.get(&dn) {
                standing.pit_stops = stop_num;
                standing.last_pit_lap = stop_lap;
            }

            // Fastest lap flag
            standing.is_fastest_lap = self.fastest_lap.map(|(fd, _)| fd == dn).unwrap_or(false);
            standing.fastest_lap = if standing.is_fastest_lap {
                self.fastest_lap.map(|(_, t)| t)
            } else {
                None
            };
        }

        // Sort by position
        self.standings.sort_by_key(|s| s.position);
    }

    pub fn fastest_s1(&self) -> Option<i32> {
        self.fastest_s1.map(|(d, _)| d)
    }
    pub fn fastest_s2(&self) -> Option<i32> {
        self.fastest_s2.map(|(d, _)| d)
    }
    pub fn fastest_s3(&self) -> Option<i32> {
        self.fastest_s3.map(|(d, _)| d)
    }

    pub fn seconds_since_update(&self) -> u64 {
        self.last_updated.elapsed().as_secs()
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

pub fn format_lap_time(secs: f64) -> String {
    let minutes = (secs / 60.0) as u32;
    let remaining = secs - (minutes as f64 * 60.0);
    format!("{}:{:06.3}", minutes, remaining)
}

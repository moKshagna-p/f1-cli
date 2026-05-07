use crate::api::{self, Driver, Session};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DriverStanding {
    pub number: i32,
    pub name: String,
    pub position: i32,
    pub gap: f64,
    pub lap_time: String,
    pub sector1: String,
    pub sector2: String,
    pub sector3: String,
    pub tyres: String,
    pub drs: bool,
    pub pit_stops: i32,
    #[allow(dead_code)]
    pub team: String,
    pub team_color: String,
    pub is_fastest_lap: bool,
    pub position_changed: i32, // -1 down, 0 no change, 1 up
}

pub struct AppState {
    pub session: Session,
    #[allow(dead_code)]
    pub drivers: Vec<Driver>,
    pub standings: Vec<DriverStanding>,
    pub fastest_lap_driver: i32,
    pub last_positions: HashMap<i32, i32>,
    pub animation_timer: u64,
}

impl AppState {
    pub fn new(session: Session, drivers: Vec<Driver>) -> Self {
        let standings = drivers
            .iter()
            .enumerate()
            .map(|(idx, d)| DriverStanding {
                number: d.driver_number,
                name: d.name_acronym.clone(),
                position: (idx + 1) as i32,
                gap: 0.0,
                lap_time: String::new(),
                sector1: String::new(),
                sector2: String::new(),
                sector3: String::new(),
                tyres: String::new(),
                drs: false,
                pit_stops: 0,
                team: d.team_name.clone(),
                team_color: d.team_colour.clone().unwrap_or_else(|| "#FFFFFF".to_string()),
                is_fastest_lap: false,
                position_changed: 0,
            })
            .collect();

        let mut last_positions = HashMap::new();
        for (idx, d) in drivers.iter().enumerate() {
            last_positions.insert(d.driver_number, (idx + 1) as i32);
        }

        Self {
            session,
            drivers,
            standings,
            fastest_lap_driver: -1,
            last_positions,
            animation_timer: 0,
        }
    }

    pub fn update(&mut self, telemetry: api::Telemetry) {
        // Update positions
        for pos in &telemetry.positions {
            if let Some(standing) = self.standings.iter_mut().find(|s| s.number == pos.driver_number) {
                if let Some(new_pos) = pos.position {
                    let old_pos = self.last_positions.get(&pos.driver_number).copied().unwrap_or(new_pos);
                    standing.position_changed = old_pos - new_pos;
                    standing.position = new_pos;
                    self.last_positions.insert(pos.driver_number, new_pos);
                }
                if let Some(gap) = pos.gap_to_leader {
                    standing.gap = gap;
                }
            }
        }

        // Update lap times
        let mut latest_laps: HashMap<i32, api::Lap> = HashMap::new();
        for lap in &telemetry.laps {
            latest_laps
                .entry(lap.driver_number)
                .and_modify(|e| {
                    if lap.lap_number > e.lap_number {
                        *e = lap.clone();
                    }
                })
                .or_insert_with(|| lap.clone());
        }

        for (driver_num, lap) in &latest_laps {
            if let Some(standing) = self.standings.iter_mut().find(|s| s.number == *driver_num) {
                if let Some(duration) = lap.lap_duration {
                    standing.lap_time = format!("1:{:05.2}", duration);
                }
                if let Some(s1) = lap.sector1_duration {
                    standing.sector1 = format!("{:05.2}", s1);
                }
                if let Some(s2) = lap.sector2_duration {
                    standing.sector2 = format!("{:05.2}", s2);
                }
                if let Some(s3) = lap.sector3_duration {
                    standing.sector3 = format!("{:05.2}", s3);
                }
            }
        }

        // Update pit stops
        for pit in &telemetry.pit_stops {
            if let Some(standing) = self.standings.iter_mut().find(|s| s.number == pit.driver_number) {
                standing.pit_stops = pit.stop_number;
            }
        }

        // Update fastest lap
        if let Some(fastest) = latest_laps
            .values()
            .min_by(|a, b| {
                a.lap_duration
                    .partial_cmp(&b.lap_duration)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            self.fastest_lap_driver = fastest.driver_number;
        }

        // Sort by position
        self.standings.sort_by_key(|s| s.position);

        // Mark fastest lap
        for standing in &mut self.standings {
            standing.is_fastest_lap = standing.number == self.fastest_lap_driver;
        }

        // Reset animation timer
        self.animation_timer = 0;
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
        self.animation_timer = self.animation_timer.saturating_add(1);
        // Fade animation after 15 ticks (3 seconds at 200ms per tick)
        if self.animation_timer > 15 {
            for standing in &mut self.standings {
                if standing.position_changed != 0 {
                    standing.position_changed = 0;
                }
            }
        }
    }
}

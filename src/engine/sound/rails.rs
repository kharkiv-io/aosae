use super::sounds_pool::SoundThreadPool;
use crate::engine::io::decoder;
use crate::engine::sound::sounds_pool;
use std::sync::Arc;
use std::thread;
use std::time;
use std::time::Duration;
#[derive(Debug)]
pub struct Rails {
    rails: Vec<Rail>,
}
#[derive(Debug)]
pub struct TrackPos {
    id: usize,
    path: String,
    pos: f64,
}
#[derive(Debug)]
pub struct Rail {
    id: usize,
    tracks: Vec<TrackPos>,
}

impl Rails {
    pub fn new() -> Self {
        Rails { rails: Vec::new() }
    }

    pub fn new_rail(&mut self) -> usize {
        let id = self.rails.len();
        self.rails.push(Rail {
            id,
            tracks: Vec::new(),
        });
        id
    }

    pub fn add_track(&mut self, id_rail: usize, ts: TrackPos) {
        if let Some(rail) = self.rails.iter_mut().find(|r| r.id == id_rail) {
            rail.tracks.push(ts);
        } else {
            println!("Cannot find rail with id -> {}", id_rail);
        }
    }

    pub fn get_rail(&self, id_rail: usize) -> Option<Rail> {
        self.rails.iter().find(|r| r.id == id_rail).map(|r| Rail {
            id: r.id,
            tracks: r
                .tracks
                .iter()
                .map(|track| TrackPos {
                    id: track.id,
                    path: track.path.clone(),
                    pos: track.pos,
                })
                .collect(),
        })
    }
}
impl TrackPos {
    pub fn new(id: usize, path: String, pos: f64) -> Self {
        TrackPos {
            id: (id),
            path: (path),
            pos: (pos),
        }
    }

    pub fn new_track_positions() -> Vec<TrackPos> {
        let tp: Vec<TrackPos> = Vec::new();
        tp
    }

    pub fn clone(&self) -> TrackPos {
        TrackPos {
            id: self.id,
            path: self.path.clone(),
            pos: self.pos,
        }
    }
}

pub fn new_timer() -> Timer {
    Timer::new()
}

pub fn timer_clone(timer: &Timer) -> Timer {
    Timer {
        start_time: timer.start_time,
    }
}
pub struct Timer {
    start_time: time::Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: time::Instant::now(),
        }
    }

    pub fn new_from_duration(duration: Duration) -> Timer {
        Timer {
            start_time: time::Instant::now() + duration,
        }
    }

    pub fn elapsed(&self) -> time::Duration {
        self.start_time.elapsed()
    }
}

pub fn calc_and_start(
    pool: Arc<Result<SoundThreadPool, anyhow::Error>>,
    rail: Rail,
    duration: f64,
    id_index: usize,
) {
    thread::spawn(move || {
        if let Ok(pool) = pool.as_ref() {
            let decoded = decoder::decode_audio_samples(rail.tracks[(id_index)].path.clone());
            if let Ok(decoded_samples) = decoded {
                loop {
                    pool.execute(
                        rail.tracks.len(),
                        decoded_samples.clone(),
                        Duration::from_secs_f64(duration),
                        Duration::from_secs_f64(rail.tracks[id_index].pos.clone()),
                    );
                    break;
                }
            } else {
                println!("Failed to decode audio samples?");
            }
        }
    });
}

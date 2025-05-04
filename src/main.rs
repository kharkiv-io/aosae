pub mod engine;
use crate::engine::cache::cache_buffer;
use crate::engine::core::rmv;
use crate::engine::io::output::{create_samples_box, play_samples_graph};
use crate::engine::sound::sounds_pool;
use crate::engine::thandler::handler;
use engine::core::rmv::ResourceManager;
use engine::io::decoder;
use engine::sound::rails::{self, Rail, Rails, TrackPos, calc_and_start, new_timer, timer_clone};
use engine::sound::sounds_pool::SoundTask;
use std::alloc::Layout;
use std::io::{self, Read};
use std::os::fd::RawFd;
use std::ptr::read;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let pool_s = Arc::new(sounds_pool::SoundThreadPool::new(2));
}

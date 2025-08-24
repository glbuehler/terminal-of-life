use std::time;
use std::sync::{RwLock, atomic::{AtomicBool, AtomicI64, AtomicU64}};
use std::collections::HashSet;

use lazy_static::lazy_static;

mod simu;
mod render;
mod input;

const SIMULATION_INTERVAL: time::Duration = time::Duration::from_millis(80);
const RENDER_INTERVAL: time::Duration = time::Duration::from_millis(20);

static CAMERA_OFFSET: (AtomicI64, AtomicI64) = (AtomicI64::new(0), AtomicI64::new(0));
static PAUSED: AtomicBool = AtomicBool::new(false);
static SIMULATION_MILLIS: AtomicU64 = AtomicU64::new(80);

lazy_static!(
    static ref WORLD: RwLock<HashSet<(i64, i64)>> = RwLock::new(HashSet::default());
);

pub fn handle_input() {
    input::handle_input();
}

pub fn spawn_simu() {

    {
        let mut world = WORLD
            .write()
            .expect("failed to acquire write lock on WORLD");
        world.insert((0, 0));
        world.insert((-1, 1));
        world.insert((1, 0));
        world.insert((1, 1));
        world.insert((1, 2));
    }
    
    std::thread::spawn(simu::loop_simu);
}

pub fn spawn_render() {
    let mut buf = vec![];
    std::thread::spawn(move || loop {
        render::draw_screen_with_buf(&mut buf);
        std::thread::sleep(RENDER_INTERVAL);
    });
}

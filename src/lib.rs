use std::time;
use std::sync::atomic::{AtomicBool, AtomicI64};

use lazy_static::lazy_static;
use dashmap::DashSet;

mod simu;
mod render;
mod input;

const SIMULATION_INTERVAL: time::Duration = time::Duration::from_millis(80);
const RENDER_INTERVAL: time::Duration = time::Duration::from_millis(20);

static CAMERA_OFFSET: (AtomicI64, AtomicI64) = (AtomicI64::new(0), AtomicI64::new(0));
static PAUSED: AtomicBool = AtomicBool::new(false);

lazy_static!(
    static ref WORLD: Box<DashSet<(i64, i64)>> = Box::new(DashSet::default());
    static ref NEXT: Box<DashSet<(i64, i64)>> = Box::new(DashSet::default());
);

pub fn handle_input() {
    input::handle_input();
}

pub fn spawn_simu() {

    WORLD.insert((0, 0));
    WORLD.insert((-1, 1));
    WORLD.insert((1, 0));
    WORLD.insert((1, 1));
    WORLD.insert((1, 2));
    
    std::thread::spawn(simu::loop_simu);
}

pub fn spawn_render() {
    let mut buf = vec![];
    std::thread::spawn(move || loop {
        render::draw_screen_with_buf(&mut buf);
        std::thread::sleep(RENDER_INTERVAL);
    });
}

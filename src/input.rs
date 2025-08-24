use std::sync::atomic::{AtomicI64, Ordering::Relaxed};

use crate::{CAMERA_OFFSET, PAUSED, SIMULATION_MILLIS, WORLD};

pub fn handle_input() {
    use crossterm::*;
    loop {
        match event::read() {
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            })) => break,
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(' '),
                ..
            })) => assert!(PAUSED.fetch_update(Relaxed, Relaxed, |b| Some(!b),).is_ok()),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            })) => assert!(
                SIMULATION_MILLIS
                    .fetch_update(Relaxed, Relaxed, |n| Some(if n > 1 { n / 2 } else { 1 }))
                    .is_ok()
            ),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            })) => assert!(
                SIMULATION_MILLIS
                    .fetch_update(Relaxed, Relaxed, |n| Some(n * 2))
                    .is_ok()
            ),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('x'),
                ..
            })) => {
                let size = crossterm::terminal::size().unwrap();
                let offset = (CAMERA_OFFSET.0.load(Relaxed), CAMERA_OFFSET.1.load(Relaxed));
                let coord = (
                    offset.0 + size.0 as i64,
                    offset.1 + size.1 as i64 / 2 * 3 as i64 - 2,
                );
                let mut world_write = WORLD.write().expect("failed to acquire WORLD write");

                if world_write.contains(&coord) {
                    world_write.remove(&coord);
                } else {
                    world_write.insert(coord);
                }
            }
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('J'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.1, |n| n + 10),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('K'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.1, |n| n - 10),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('L'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.0, |n| n + 10),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('H'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.0, |n| n - 10),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('j'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.1, |n| n + 1),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('k'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.1, |n| n - 1),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('l'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.0, |n| n + 1),
            Ok(event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('h'),
                ..
            })) => update_atomic(&CAMERA_OFFSET.0, |n| n - 1),
            _ => (),
        }
    }
}

fn update_atomic(at: &AtomicI64, mut f: impl FnMut(i64) -> i64) {
    use std::sync::atomic::Ordering;
    assert!(
        at.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| Some(f(n)),)
            .is_ok()
    );
}

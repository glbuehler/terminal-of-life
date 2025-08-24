use crate::{PAUSED, SIMULATION_MILLIS, WORLD};
use core::time;
use std::collections::HashSet;
use std::sync::atomic::Ordering::Relaxed;

pub fn loop_simu() {
    let mut next_world = HashSet::new();
    loop {
        next_world.clear();
        let world_copy;
        {
            let world_read = WORLD.read().expect("failed to acquire read on WORLD");
            world_copy = world_read.clone();
        }
        for (x, y) in world_copy.iter().cloned() {
            let mut neighbors = 0;
            for (i, j) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if world_copy.contains(&(x + i, y + j)) {
                    neighbors += 1;
                } else if !next_world.contains(&(x + i, y + j)) {
                    let mut dead_neighbors = 0;
                    for (k, l) in &[
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ] {
                        if world_copy.contains(&(x + i + k, y + j + l)) {
                            dead_neighbors += 1;
                        }
                    }
                    if dead_neighbors == 3 {
                        next_world.insert((x + i, y + j));
                    }
                }
            }
            if neighbors >= 2 && neighbors <= 3 {
                next_world.insert((x, y));
            }
        }

        {
            let mut world_write = WORLD.write().expect("failed to acquire WORLD write");
            std::mem::swap(&mut *world_write, &mut next_world);
        }

        let mut paused;
        loop {
            let millis = SIMULATION_MILLIS.load(Relaxed);
            std::thread::sleep(time::Duration::from_millis(millis));
            paused = PAUSED.load(Relaxed);
            if !paused {
                break;
            }
        }
    }
}

fn check_coordinates(
    iter: impl Iterator<Item = (i64, i64)>,
    mut cell_cb: impl FnMut(usize, i64, i64),
    mut neighbor_cb: impl FnMut(i64, i64),
) {
    let world_read = WORLD.read().expect("failed to acquire WORLD read");
    for (x, y) in iter {
        let mut neighbors = 0;
        for (i, j) in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            if world_read.contains(&(x + i, y + j)) {
                neighbors += 1;
            }
            neighbor_cb(x + i, y + j);
        }
        cell_cb(neighbors, x, y);
    }
}

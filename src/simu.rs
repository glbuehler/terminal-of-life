use std::sync::atomic::Ordering::Relaxed;
use std::collections::HashSet;
use crate::{WORLD, NEXT, PAUSED, SIMULATION_INTERVAL};

pub fn loop_simu() {

    let mut working_set = HashSet::new();
    loop {
        working_set.clear();
        NEXT.clear();
        check_coordinates(
            WORLD.iter().map(|t| (t.0, t.1)),
            |n, x, y| if n >= 2 && n <= 3 {
                NEXT.insert((x, y));
            },
            |x, y| if !WORLD.contains(&(x, y)) { working_set.insert((x, y)); },
        );
        check_coordinates(
            working_set.drain(),
            |n, x, y| if n == 3 { NEXT.insert((x, y)); },
            |_, _| {},
        );

        WORLD.clear();
        for c in NEXT.iter().map(|t| (t.0, t.1)) {
            WORLD.insert(c);
        }

        let mut paused;
        loop {
            std::thread::sleep(SIMULATION_INTERVAL);
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
    mut neighbor_cb: impl FnMut(i64 ,i64),
) {
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
            (1, 1)
        ] {
            if WORLD.contains(&(x + i, y + j)) {
                neighbors += 1;
            }
            neighbor_cb(x + i, y + j);
        }
        cell_cb(neighbors, x, y);
    }
}


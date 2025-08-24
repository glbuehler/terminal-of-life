use crate::{CAMERA_OFFSET, WORLD};
use std::io::Write;

pub fn draw_screen_with_buf(buf: &mut Vec<u8>) {
    let size = crossterm::terminal::size();
    assert!(size.is_ok());
    let size = size.unwrap();
    let size = (size.0 as i64, size.1 as i64);
    let offset = (
        CAMERA_OFFSET.0.load(std::sync::atomic::Ordering::Relaxed),
        CAMERA_OFFSET.1.load(std::sync::atomic::Ordering::Relaxed),
    );

    buf.clear();
    buf.extend(b"\x1b[?25l\x1b[H");
    for j in 0..size.1 {
        for i in 0..size.0 {
            if i == size.0 / 2 && j == size.1 / 2 {
                buf.extend("\x1b[32;44m".as_bytes());
                buf.extend(
                    get_symbol_2x3(offset.0 + i * 2, offset.1 + (j - 1) * 3)
                        .to_string()
                        .as_bytes(),
                );
                buf.extend("\x1b[0m".as_bytes());
                continue;
            }
            buf.extend(
                get_symbol_2x3(offset.0 + i * 2, offset.1 + (j - 1) * 3)
                    .to_string()
                    .as_bytes(),
            );
        }
        buf.extend(b"\r\n");
    }

    assert!(std::io::stdout().write_all(&buf).is_ok());
}

fn get_symbol_2x2(x: i64, y: i64) -> char {
    let values;
    {
        let world_read = WORLD.read().expect("failed to acquire WORLD read");
        values = (
            world_read.contains(&(x, y)),
            world_read.contains(&(x + 1, y)),
            world_read.contains(&(x, y + 1)),
            world_read.contains(&(x + 1, y + 1)),
        );
    }
    match values {
        (false, false, false, false) => ' ',
        (true, false, false, false) => '▘',
        (false, true, false, false) => '▝',
        (false, false, true, false) => '▖',
        (false, false, false, true) => '▗',
        (true, true, false, false) => '▀',
        (true, false, true, false) => '▌',
        (false, true, false, true) => '▐',
        (false, false, true, true) => '▄',
        (true, true, true, false) => '▛',
        (true, false, true, true) => '▙',
        (false, true, true, true) => '▟',
        (true, true, false, true) => '▜',
        (true, false, false, true) => '▚',
        (false, true, true, false) => '▞',
        (true, true, true, true) => '█',
    }
}

fn get_symbol_2x3(x: i64, y: i64) -> char {
    const SYMBOLS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', '🬏', '🬐',
        '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', '🬞', '🬟', '🬠', '🬡',
        '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', '🬭', '🬮', '🬯', '🬰', '🬱', '🬲',
        '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█',
    ];

    let mut values = [false; 6];
    {
        let world_read = WORLD.read().expect("failed to acquire WORLD read");
        values[0] = world_read.contains(&(x + 0, y + 0));
        values[1] = world_read.contains(&(x + 1, y + 0));
        values[2] = world_read.contains(&(x + 0, y + 1));
        values[3] = world_read.contains(&(x + 1, y + 1));
        values[4] = world_read.contains(&(x + 0, y + 2));
        values[5] = world_read.contains(&(x + 1, y + 2));
    }

    let mut idx = 0;
    idx |= if values[0] { 1 } else { 0 } << 0;
    idx |= if values[1] { 1 } else { 0 } << 1;
    idx |= if values[2] { 1 } else { 0 } << 2;
    idx |= if values[3] { 1 } else { 0 } << 3;
    idx |= if values[4] { 1 } else { 0 } << 4;
    idx |= if values[5] { 1 } else { 0 } << 5;

    SYMBOLS[idx]
}

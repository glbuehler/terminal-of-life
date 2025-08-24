use gol::*;

fn main() {
    assert!(crossterm::terminal::enable_raw_mode().is_ok());
    assert!(
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen,).is_ok()
    );

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        exit();
        hook(info);
    }));

    spawn_render();
    spawn_simu();

    handle_input();

    exit();
}

fn exit() {
    assert!(
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen).is_ok()
    );
    assert!(crossterm::terminal::disable_raw_mode().is_ok());
    println!("\x1b[?25h");
}

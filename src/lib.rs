use jay_config::{config, quit, reload};
use jay_config::input::get_default_seat;
use jay_config::keyboard::mods::SUPER;
use jay_config::keyboard::syms::{SYM_q, SYM_r};


fn configure() {
    let seat = get_default_seat();
    // Create a key binding to exit the compositor.
    seat.bind(SUPER | SYM_q, || quit());
    // Reload the configuration.
    seat.bind(SUPER | SYM_r, || reload());
}

config!(configure);

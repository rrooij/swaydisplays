mod display;

use cursive::views::{Dialog, LinearLayout, RadioGroup};
use display::{get_display_modes, set_output, Display, Mode};

fn main() {
    let displays: Vec<(Display, Mode)> = get_display_modes();
    let displays_for_buttons = displays.clone();
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut display_group = RadioGroup::<(Display, Mode)>::new();
    let mut linear = LinearLayout::vertical();

    for display in displays {
        let current_mode = display.1.clone();
        let display_name = format!(
            "{} {}x{} - {}hz",
            display.0.name, current_mode.width, current_mode.height, current_mode.refresh / 1000
        );
        let mut button = display_group.button(display.clone(), display_name);
        match display.0.current_mode {
            Some(mode) => {
                if mode.height == current_mode.height && mode.width == current_mode.width && mode.refresh == current_mode.refresh {
                   button.select();
                }
            },
            None => {}
        }
        linear.add_child(button);
    }

    siv.add_layer(
        Dialog::new()
            .content(linear)
            .title("Displays")
            .button("Set display", move |_s| {
                let display_mode = displays_for_buttons[display_group.selected_id()].clone();
                set_output(display_mode);
            })
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
}

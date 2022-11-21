mod display;

use cursive::view::Nameable;
use cursive::views::{Checkbox, Dialog, LinearLayout, RadioGroup, ScrollView, TextView, ViewRef};
use display::{get_displays, Display, Mode};
use std::collections::HashMap;

fn main() {
    let displays = get_displays();
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    siv.set_user_data(displays.clone());
    siv.add_global_callback('q', |s| s.quit());

    let mut linear = LinearLayout::vertical();

    for (display_name, display) in displays {
        let display_group = RadioGroup::<Mode>::new();
        let mut display_horizontal_layout = LinearLayout::horizontal();
        let mut display_clone = display.clone();
        let mut vertical_linear = LinearLayout::vertical().with_name(display_clone.name.clone());
        let name = display_clone.name.clone();
        display_horizontal_layout.add_child(TextView::new(name));
        if display.active {
            add_modes(
                &mut vertical_linear.get_mut(),
                &mut display_clone,
                display_group.clone(),
            );
        }
        let checkbox_display_group = display_group.clone();
        let checkbox = Checkbox::new()
            .with_checked(display_clone.active)
            .on_change(move |s, checked| {
                let user_data = s
                    .with_user_data(|user_data: &mut HashMap<String, Display>| user_data.clone())
                    .unwrap();
                let selected_display_first = &user_data[&display_name];
                let mut view: ViewRef<LinearLayout> =
                    s.find_name(&selected_display_first.name).unwrap();
                s.with_user_data(|user_data: &mut HashMap<String, Display>| {
                    let mut selected_display = &mut user_data.get_mut(&display_name).unwrap();
                    match checked {
                        true => {
                            selected_display.active = true;
                            add_modes(&mut *view, selected_display, checkbox_display_group.clone());
                        }
                        false => {
                            selected_display.active = false;
                            view.clear();
                        }
                    }
                })
                .unwrap()
            });
        display_horizontal_layout.add_child(checkbox);
        linear.add_child(display_horizontal_layout);
        linear.add_child(vertical_linear);
    }

    siv.add_layer(
        Dialog::new()
            .content(ScrollView::new(linear).show_scrollbars(true))
            .title("Displays")
            .button("Set display", |s| {
                s.with_user_data(|user_data: &mut HashMap<String, Display>| {
                    for display in user_data.values() {
                        let display_loop = display.clone();
                        display_loop.process();
                    }
                });
            })
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
}

fn add_modes(
    vertical_linear: &mut LinearLayout,
    display: &mut Display,
    mut display_group: RadioGroup<Mode>,
) {
    for mode in &display.modes {
        let display_name = format!(
            "{} {}x{} - {}hz",
            display.name,
            mode.width,
            mode.height,
            mode.refresh / 1000
        );

        let mut button = display_group.button(mode.clone(), display_name.clone());
        let mode_clone = mode.clone();
        if display.clone().is_current_mode(mode_clone) {
            button.select();
        }
        vertical_linear.add_child(button);
    }
    let display_name = display.name.clone();
    display_group.set_on_change(move |s, display_mode: &Mode| {
        s.with_user_data(|user_data: &mut HashMap<String, Display>| {
            let mut selected_display = user_data.get_mut(&display_name).unwrap();
            selected_display.current_mode = Some(display_mode.clone());
        });
    });
}

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use wxdragon::prelude::*;

use crate::analog::AnalogClock;
use crate::digital::DigitalClock;
use crate::traits::DrawClock;

mod analog;
mod digital;
mod traits;

const ID_SHOW_SECONDS: Id = ID_HIGHEST + 1;
const ID_ALWAYS_ON_TOP: Id = ID_HIGHEST + 2;
const ID_VIEW_ANALOG: Id = ID_HIGHEST + 3;
const ID_VIEW_DIGITAL: Id = ID_HIGHEST + 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayMode {
    Analog,
    Digital,
}

#[derive(Debug, Clone, Copy)]
struct ClockState {
    show_seconds: bool,
    mode: DisplayMode,
    title_bar_hidden: bool,
}

const DOUBLE_CLICK_THRESHOLD: Duration = Duration::from_millis(450);
const DOUBLE_CLICK_MAX_DISTANCE: i32 = 6;

fn set_title_bar_visible(frame: &Frame, visible: bool) {
    let mut style = frame.get_style_raw();
    let title_bits = FrameStyle::Caption.bits()
        | FrameStyle::SystemMenu.bits()
        | FrameStyle::MinimizeBox.bits()
        | FrameStyle::MaximizeBox.bits()
        | FrameStyle::CloseBox.bits();

    if visible {
        style |= title_bits;
    } else {
        style &= !title_bits;
    }
    frame.set_style_raw(style);
}

fn draw_clock(panel: &Panel, state: ClockState) {
    if state.mode == DisplayMode::Analog {
        let clock = AnalogClock::new(state);
        clock.draw(&panel);
    } else {
        let clock = DigitalClock::new(state);
        clock.draw(&panel);
    }
}

fn main() {
    wxdragon::main(|app| {
        let frame = Frame::builder()
            .with_title("Clock")
            .with_size(Size::new(260, 300))
            .build();
        let panel = Panel::builder(&frame).build();
        panel.set_background_style(BackgroundStyle::Paint);

        let file_menu = Menu::builder()
            .append_item(ID_EXIT, "E&xit\tAlt+F4", "")
            .build();
        let view_menu = Menu::builder()
            .append_radio_item(ID_VIEW_ANALOG, "Analog", "")
            .append_radio_item(ID_VIEW_DIGITAL, "Digital", "")
            .build();
        let options_menu = Menu::builder()
            .append_check_item(ID_SHOW_SECONDS, "Show seconds", "")
            .append_check_item(ID_ALWAYS_ON_TOP, "Always on top", "")
            .build();
        let menubar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(view_menu, "&View")
            .append(options_menu, "&Options")
            .build();
        frame.set_menu_bar(menubar);

        if let Some(mb) = frame.get_menu_bar() {
            mb.check_item(ID_SHOW_SECONDS, true);
            mb.check_item(ID_ALWAYS_ON_TOP, false);
            mb.check_item(ID_VIEW_ANALOG, true);
            mb.check_item(ID_VIEW_DIGITAL, false);
        }

        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(&panel, 1, SizerFlag::Expand, 0);
        frame.set_sizer(sizer, true);

        let state = Rc::new(RefCell::new(ClockState {
            show_seconds: true,
            mode: DisplayMode::Analog,
            title_bar_hidden: false,
        }));

        let panel_for_paint = panel;
        let state_for_paint = state.clone();
        panel.on_paint(move |_| {
            draw_clock(&panel_for_paint, *state_for_paint.borrow());
        });

        let frame_for_dbl = frame;
        let state_for_dbl = state.clone();
        let last_click = Rc::new(RefCell::new(None::<(Instant, i32, i32)>));
        let last_click_for_handler = last_click.clone();
        panel.on_mouse_left_down(move |event| {
            let now = Instant::now();
            let mut click = last_click_for_handler.borrow_mut();
            let (x, y) = match event {
                WindowEventData::MouseButton(mouse) => mouse
                    .get_position()
                    .map(|p| (p.x, p.y))
                    .unwrap_or((-9999, -9999)),
                _ => (-9999, -9999),
            };

            if let Some((prev_time, prev_x, prev_y)) = *click {
                let in_time = now.duration_since(prev_time) <= DOUBLE_CLICK_THRESHOLD;
                let near_pos = (x - prev_x).abs() <= DOUBLE_CLICK_MAX_DISTANCE
                    && (y - prev_y).abs() <= DOUBLE_CLICK_MAX_DISTANCE;
                if in_time && near_pos {
                    let mut s = state_for_dbl.borrow_mut();
                    s.title_bar_hidden = !s.title_bar_hidden;
                    set_title_bar_visible(&frame_for_dbl, !s.title_bar_hidden);
                    *click = None;
                    return;
                }
            }

            *click = Some((now, x, y));
        });

        let timer = Rc::new(Timer::new(&panel));
        let panel_for_tick = panel;
        timer.on_tick(move |_| {
            panel_for_tick.refresh(false, None);
        });
        timer.start(1000, false);

        let frame_for_menu = frame;
        let panel_for_menu = panel;
        let state_for_menu = state.clone();
        frame.on_menu_selected(move |event| {
            let id = event.get_id();
            if id == ID_EXIT {
                frame_for_menu.close(true);
                return;
            }
            if id == ID_SHOW_SECONDS {
                if let Some(mb) = frame_for_menu.get_menu_bar() {
                    state_for_menu.borrow_mut().show_seconds = mb.is_item_checked(ID_SHOW_SECONDS);
                }
                panel_for_menu.refresh(false, None);
                return;
            }
            if id == ID_ALWAYS_ON_TOP {
                if let Some(mb) = frame_for_menu.get_menu_bar() {
                    let checked = mb.is_item_checked(ID_ALWAYS_ON_TOP);
                    let mut style = frame_for_menu.get_style_raw();
                    let stay_on_top = FrameStyle::StayOnTop.bits();
                    if checked {
                        style |= stay_on_top;
                    } else {
                        style &= !stay_on_top;
                    }
                    frame_for_menu.set_style_raw(style);
                }
                return;
            }
            if id == ID_VIEW_ANALOG {
                state_for_menu.borrow_mut().mode = DisplayMode::Analog;
                panel_for_menu.refresh(false, None);
                return;
            }
            if id == ID_VIEW_DIGITAL {
                state_for_menu.borrow_mut().mode = DisplayMode::Digital;
                panel_for_menu.refresh(false, None);
            }
        });

        let timer_for_close = timer.clone();
        frame.on_close(move |event| {
            timer_for_close.stop();
            event.skip(true);
        });

        app.set_top_window(&frame);
        frame.show(true);
        frame.centre();
    })
    .unwrap();
}

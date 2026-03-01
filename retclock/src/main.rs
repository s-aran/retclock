use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use wxdragon::prelude::*;

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
}

fn draw_hand(dc: &AutoBufferedPaintDC, cx: i32, cy: i32, angle: f64, length: f64, width: i32, color: Colour) {
    let x = cx as f64 + length * angle.sin();
    let y = cy as f64 - length * angle.cos();
    dc.set_pen(color, width, PenStyle::Solid);
    dc.draw_line(cx, cy, x.round() as i32, y.round() as i32);
}

fn draw_analog_clock(panel: &Panel, state: ClockState) {
    let dc = AutoBufferedPaintDC::new(panel);
    let size = panel.get_client_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let footer_h = 34;
    let dial_h = (height - footer_h).max(24);
    let radius = ((width.min(dial_h) / 2) - 10).max(10);
    let cx = width / 2;
    let cy = dial_h / 2;

    dc.set_background(Colour::rgb(192, 192, 192));
    dc.clear();

    dc.set_brush(Colour::rgb(236, 236, 236), BrushStyle::Solid);
    dc.set_pen(Colour::rgb(64, 64, 64), 2, PenStyle::Solid);
    dc.draw_circle(cx, cy, radius);

    for i in 0..60 {
        let angle = (i as f64) * (2.0 * PI) / 60.0;
        let inner = if i % 5 == 0 {
            radius as f64 * 0.84
        } else {
            radius as f64 * 0.90
        };
        let outer = radius as f64 * 0.97;
        let x1 = cx as f64 + inner * angle.sin();
        let y1 = cy as f64 - inner * angle.cos();
        let x2 = cx as f64 + outer * angle.sin();
        let y2 = cy as f64 - outer * angle.cos();
        dc.set_pen(
            Colour::rgb(32, 32, 32),
            if i % 5 == 0 { 2 } else { 1 },
            PenStyle::Solid,
        );
        dc.draw_line(
            x1.round() as i32,
            y1.round() as i32,
            x2.round() as i32,
            y2.round() as i32,
        );
    }

    let now = DateTime::now();
    let hour = (now.hour() % 12) as f64 + now.minute() as f64 / 60.0 + now.second() as f64 / 3600.0;
    let minute = now.minute() as f64 + now.second() as f64 / 60.0;
    let second = now.second() as f64;

    draw_hand(
        &dc,
        cx,
        cy,
        hour * (2.0 * PI) / 12.0,
        radius as f64 * 0.55,
        4,
        colours::BLACK,
    );
    draw_hand(
        &dc,
        cx,
        cy,
        minute * (2.0 * PI) / 60.0,
        radius as f64 * 0.78,
        3,
        colours::BLACK,
    );
    if state.show_seconds {
        draw_hand(
            &dc,
            cx,
            cy,
            second * (2.0 * PI) / 60.0,
            radius as f64 * 0.84,
            1,
            Colour::rgb(180, 0, 0),
        );
    }

    dc.set_pen(colours::BLACK, 1, PenStyle::Solid);
    dc.set_brush(colours::BLACK, BrushStyle::Solid);
    dc.draw_circle(cx, cy, 3);

    if let Some(font) = Font::builder()
        .with_point_size(9)
        .with_family(FontFamily::Swiss)
        .build()
    {
        dc.set_font(&font);
    }
    dc.set_text_foreground(colours::BLACK);
    let text = format!(
        "{:04}-{:02}-{:02}  {:02}:{:02}:{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    dc.draw_text(&text, 12, height - 24);
}

fn draw_digital_clock(panel: &Panel, state: ClockState) {
    let dc = AutoBufferedPaintDC::new(panel);
    let size = panel.get_client_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let now = DateTime::now();

    dc.set_background(Colour::rgb(192, 192, 192));
    dc.clear();

    dc.set_pen(Colour::rgb(96, 96, 96), 1, PenStyle::Solid);
    dc.set_brush(Colour::rgb(236, 236, 236), BrushStyle::Solid);
    dc.draw_rectangle(12, 12, width - 24, height - 24);

    let time_text = if state.show_seconds {
        format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
    } else {
        format!("{:02}:{:02}", now.hour(), now.minute())
    };
    let date_text = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());

    if let Some(time_font) = Font::builder()
        .with_point_size(28)
        .with_family(FontFamily::Modern)
        .with_weight(FontWeight::Bold)
        .build()
    {
        dc.set_font(&time_font);
    }
    dc.set_text_foreground(colours::BLACK);
    let (tw, th) = dc.get_text_extent(&time_text);
    let tx = (width - tw) / 2;
    let ty = (height - th) / 2 - 22;
    dc.draw_text(&time_text, tx, ty);

    if let Some(date_font) = Font::builder()
        .with_point_size(11)
        .with_family(FontFamily::Swiss)
        .build()
    {
        dc.set_font(&date_font);
    }
    let (dw, _) = dc.get_text_extent(&date_text);
    dc.draw_text(&date_text, (width - dw) / 2, ty + th + 12);
}

fn draw_clock(panel: &Panel, state: ClockState) {
    if state.mode == DisplayMode::Analog {
        draw_analog_clock(panel, state);
    } else {
        draw_digital_clock(panel, state);
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

        let file_menu = Menu::builder().append_item(ID_EXIT, "E&xit\tAlt+F4", "").build();
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
        }));

        let panel_for_paint = panel;
        let state_for_paint = state.clone();
        panel.on_paint(move |_| {
            draw_clock(&panel_for_paint, *state_for_paint.borrow());
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

use std::f64::consts::PI;

use wxdragon::*;

use crate::{ClockState, traits::DrawClock};

pub struct AnalogClock<'a> {
    state: ClockState,

    panel: &'a Panel,
    width: i32,
    height: i32,
}

impl<'a> DrawClock<'a> for AnalogClock<'a> {
    fn get_panel(&self) -> &Panel {
        self.panel
    }

    fn draw(&self) {
        self.draw_analog_clock(self.get_panel());
    }
}

impl<'a> AnalogClock<'a> {
    pub fn new(panel: &'a Panel, state: ClockState) -> Self {
        let size = panel.get_client_size();
        Self {
            state,
            panel,
            width: size.width.max(1),
            height: size.height.max(1),
        }
    }

    fn draw_hand(
        dc: &AutoBufferedPaintDC,
        cx: i32,
        cy: i32,
        angle: f64,
        length: f64,
        width: i32,
        color: Colour,
    ) {
        let x = cx as f64 + length * angle.sin();
        let y = cy as f64 - length * angle.cos();
        dc.set_pen(color, width, PenStyle::Solid);
        dc.draw_line(cx, cy, x.round() as i32, y.round() as i32);
    }

    fn draw_analog_clock(&self, panel: &Panel) {
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
        let hour =
            (now.hour() % 12) as f64 + now.minute() as f64 / 60.0 + now.second() as f64 / 3600.0;
        let minute = now.minute() as f64 + now.second() as f64 / 60.0;
        let second = now.second() as f64;

        Self::draw_hand(
            &dc,
            cx,
            cy,
            hour * (2.0 * PI) / 12.0,
            radius as f64 * 0.55,
            4,
            colours::BLACK,
        );
        Self::draw_hand(
            &dc,
            cx,
            cy,
            minute * (2.0 * PI) / 60.0,
            radius as f64 * 0.78,
            3,
            colours::BLACK,
        );

        if self.state.show_seconds {
            Self::draw_hand(
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
}

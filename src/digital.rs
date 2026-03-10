use wxdragon::*;

use crate::{
    ClockState,
    consts::{FRAME_HEIGHT, FRAME_WIDTH},
    traits::{Clock, Drawable},
};

pub struct DigitalClock<'a> {
    state: ClockState,

    panel: &'a Panel,
    width: i32,
    height: i32,

    time_font_pt: i32,
    date_font_pt: i32,
}

impl<'a> Drawable for DigitalClock<'a> {
    fn draw(&self) {
        self.draw_digital_clock(self.get_panel());
    }
}

impl<'a> Clock<'a> for DigitalClock<'a> {
    fn new(panel: &'a Panel, state: ClockState) -> Self {
        let size = panel.get_client_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let width_ratio = width as f64 / FRAME_WIDTH as f64;
        let height_ratio = height as f64 / FRAME_HEIGHT as f64;
        let ratio = width_ratio.min(height_ratio);

        let time_font_pt = (32.0 * ratio).round().max(12.0) as i32;
        let date_font_pt = (26.0 * ratio).round().max(10.0) as i32;

        Self {
            state,
            panel,
            width,
            height,

            time_font_pt,
            date_font_pt,
        }
    }

    fn get_panel(&self) -> &Panel {
        self.panel
    }
}

impl<'a> DigitalClock<'a> {
    fn draw_digital_clock(&self, panel: &Panel) {
        let dc = AutoBufferedPaintDC::new(panel);
        let now = DateTime::now();

        dc.set_background(Colour::rgb(192, 192, 192));
        dc.clear();

        dc.set_pen(Colour::rgb(96, 96, 96), 1, PenStyle::Solid);
        dc.set_brush(Colour::rgb(236, 236, 236), BrushStyle::Solid);

        let time_text = if self.state.show_seconds {
            format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
        } else {
            format!("{:02}:{:02}", now.hour(), now.minute())
        };
        let date_text = format!("{:04}/{:02}/{:02}", now.year(), now.month(), now.day());

        let time_font = Font::builder()
            .with_point_size(self.time_font_pt)
            .with_family(FontFamily::Modern)
            .with_weight(FontWeight::Bold)
            .build();
        let date_font = Font::builder()
            .with_point_size(self.date_font_pt)
            .with_family(FontFamily::Swiss)
            .build();

        dc.set_text_foreground(colours::BLACK);

        if let Some(font) = &time_font {
            dc.set_font(font);
        }
        let (tw, th) = dc.get_text_extent(&time_text);

        if let Some(font) = &date_font {
            dc.set_font(font);
        }
        let (dw, dh) = dc.get_text_extent(&date_text);

        let gap = ((self.height as f64) * 0.04).round().max(6.0) as i32;
        let total_h = th + gap + dh;
        let top = (self.height - total_h) / 2;

        let time_x = (self.width - tw) / 2;
        let time_y = top;
        let date_x = (self.width - dw) / 2;
        let date_y = top + th + gap;

        if let Some(font) = &time_font {
            dc.set_font(font);
        }
        dc.draw_text(&time_text, time_x, time_y);

        if let Some(font) = &date_font {
            dc.set_font(font);
        }
        dc.draw_text(&date_text, date_x, date_y);
    }
}

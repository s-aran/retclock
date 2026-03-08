use wxdragon::*;

use crate::{ClockState, traits::DrawClock};

pub struct DigitalClock {
    state: ClockState,
}

impl DrawClock for DigitalClock {
    fn draw(&self, panel: &Panel) {
        self.draw_digital_clock(panel);
    }
}

impl DigitalClock {
    pub fn new(state: ClockState) -> Self {
        Self { state }
    }

    pub fn draw_digital_clock(&self, panel: &Panel) {
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

        let time_text = if self.state.show_seconds {
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
}

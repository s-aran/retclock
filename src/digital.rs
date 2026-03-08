use wxdragon::*;

use crate::{ClockState, traits::DrawClock};

pub struct DigitalClock<'a> {
    state: ClockState,

    panel: &'a Panel,
    width: i32,
    height: i32,
}

impl<'a> DrawClock<'a> for DigitalClock<'a> {
    fn get_panel(&self) -> &Panel {
        self.panel
    }

    fn draw(&self) {
        self.draw_digital_clock(self.get_panel());
    }
}

impl<'a> DigitalClock<'a> {
    pub fn new(panel: &'a Panel, state: ClockState) -> Self {
        let size = panel.get_client_size();
        Self {
            state,
            panel,
            width: size.width.max(1),
            height: size.height.max(1),
        }
    }

    fn draw_time(&self, dc: &AutoBufferedPaintDC, now: &DateTime) {
        let time_text = if self.state.show_seconds {
            format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
        } else {
            format!("{:02}:{:02}", now.hour(), now.minute())
        };

        if let Some(time_font) = Font::builder()
            .with_point_size(32)
            .with_family(FontFamily::Modern)
            .with_weight(FontWeight::Bold)
            .build()
        {
            dc.set_font(&time_font);
        }
        dc.set_text_foreground(colours::BLACK);

        let (tw, th) = dc.get_text_extent(&time_text);
        let tx = (self.width - tw) / 2;
        let ty = (self.height - th) / 2 - 32;
        dc.draw_text(&time_text, tx, ty);
    }

    fn draw_date(&self, dc: &AutoBufferedPaintDC, now: &DateTime) {
        let date_text = format!("{:04}/{:02}/{:02}", now.year(), now.month(), now.day());

        if let Some(date_font) = Font::builder()
            .with_point_size(26)
            .with_family(FontFamily::Swiss)
            .build()
        {
            dc.set_font(&date_font);
        }
        let (dw, _) = dc.get_text_extent(&date_text);

        let (_, th) = dc.get_text_extent(&date_text);
        let ty = (self.height - th) / 2 - 26;
        dc.draw_text(&date_text, (self.width - dw) / 2, ty + th + 12);
    }

    fn draw_digital_clock(&self, panel: &Panel) {
        let dc = AutoBufferedPaintDC::new(panel);
        let now = DateTime::now();

        dc.set_background(Colour::rgb(192, 192, 192));
        dc.clear();

        dc.set_pen(Colour::rgb(96, 96, 96), 1, PenStyle::Solid);
        dc.set_brush(Colour::rgb(236, 236, 236), BrushStyle::Solid);

        self.draw_time(&dc, &now);
        self.draw_date(&dc, &now);
    }
}

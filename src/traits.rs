use wxdragon::Panel;

use crate::ClockState;

pub trait Drawable {
    fn draw(&self);
}

pub trait Clock<'a>: Drawable {
    fn new(panel: &'a Panel, state: ClockState) -> Self;

    fn get_panel(&self) -> &Panel;
}

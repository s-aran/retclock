use wxdragon::Panel;

pub trait DrawClock<'a> {
    fn get_panel(&self) -> &Panel;

    fn draw(&self);
}

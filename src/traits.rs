use wxdragon::Panel;

pub trait DrawClock {
    fn draw(&self, panel: &Panel);
}

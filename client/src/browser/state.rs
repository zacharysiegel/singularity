use crate::browser::available::Available;
use crate::browser::former::Former;
use crate::browser::participating::Participating;
use crate::input::KeyPressResult;
use crate::locked_switch::LockedSwitch;
use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

#[derive(Debug)]
pub struct BrowserState {
    pub domain_switch: LockedSwitch<BrowserDomain>,
}

impl BrowserState {
    pub const DEFAULT: BrowserState = BrowserState {
        domain_switch: LockedSwitch::new(BrowserDomain::DEFAULT),
    };
}

pub trait BrowserDomainTrait {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle);
    fn key_press(&self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult;
}

#[derive(Debug, Copy, Clone)]
pub enum BrowserDomain {
    Former(Former),
    Participating(Participating),
    Available(Available),
}

impl BrowserDomain {
    pub const DEFAULT: BrowserDomain = BrowserDomain::Participating(Participating);
}

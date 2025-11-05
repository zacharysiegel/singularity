use crate::browser::draw;
use crate::locked_switch::LockedSwitch;
use raylib::drawing::RaylibDrawHandle;

#[derive(Debug)]
pub struct BrowserState {
    pub domain_switch: LockedSwitch<BrowserDomain>,
}

impl BrowserState {
    pub const DEFAULT: BrowserState = BrowserState {
        domain_switch: LockedSwitch::new(BrowserDomain::Participating),
    };
}

#[derive(Debug, Copy, Clone)]
pub enum BrowserDomain {
    Former,
    Participating,
    Available,
}

impl Default for BrowserDomain {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl BrowserDomain {
    pub const DEFAULT: BrowserDomain = BrowserDomain::Participating;

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        match self {
            BrowserDomain::Former => draw::draw_former(rl_draw),
            BrowserDomain::Participating => draw::draw_former(rl_draw),
            BrowserDomain::Available => draw::draw_former(rl_draw),
        }
    }
}

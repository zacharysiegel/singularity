use crate::facility::control_center::ControlCenter;
use crate::facility::metal_extractor::MetalExtractor;
use crate::facility::oil_extractor::OilExtractor;
use crate::map::{HexCoord, RenderCoord};
use raylib::drawing::RaylibDrawHandle;

#[derive(Debug, Copy, Clone)]
pub enum Facility<'a> {
    ControlCenter(&'a ControlCenter),
    MetalExtractor(&'a MetalExtractor),
    OilExtractor(&'a OilExtractor),
}

impl<'a> Facility<'a> {
    pub fn location(&self) -> HexCoord {
        match self {
            Facility::ControlCenter(facility) => facility.location(),
            Facility::MetalExtractor(facility) => facility.location(),
            Facility::OilExtractor(facility) => facility.location(),
        }
    }

    pub fn state(&self) -> FacilityState {
        match self {
            Facility::ControlCenter(facility) => facility.state(),
            Facility::MetalExtractor(facility) => facility.state(),
            Facility::OilExtractor(facility) => facility.state(),
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) {
        match self {
            Facility::ControlCenter(facility) => facility.draw(rl_draw, render_coord),
            Facility::MetalExtractor(facility) => facility.draw(rl_draw, render_coord),
            Facility::OilExtractor(facility) => facility.draw(rl_draw, render_coord),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Facility::ControlCenter(_) => "Control Center",
            Facility::MetalExtractor(_) => "Metal Extractor",
            Facility::OilExtractor(_) => "Oil Extractor",
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum FacilityState {
    #[default]
    Operating = 0,
    Placing,
    Destroyed,
}

pub trait FacilityTrait {
    fn location(&self) -> HexCoord;
    fn state(&self) -> FacilityState;
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord);
    fn facility<'a>(&'a self) -> Facility<'a>;
}

#[derive(Debug, Default)]
pub struct FacilityCollection {
    pub control_center_vec: Vec<ControlCenter>,
    pub metal_extractor_vec: Vec<MetalExtractor>,
    pub oil_extractor_vec: Vec<OilExtractor>,
}

impl FacilityCollection {
    pub fn all_facilities<'a>(&'a self) -> Vec<Facility<'a>> {
        let mut output: Vec<Facility> = Vec::with_capacity(
            self.control_center_vec.len() + self.metal_extractor_vec.len() + self.oil_extractor_vec.len(),
        );
        output.extend(self.control_center_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output.extend(self.metal_extractor_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output.extend(self.oil_extractor_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output
    }

    pub fn at<'a>(&'a self, hex_coord: HexCoord) -> Option<Facility<'a>> {
        for facility in self.all_facilities() {
            if hex_coord == facility.location() {
                return Some(facility);
            }
        }
        None
    }
}

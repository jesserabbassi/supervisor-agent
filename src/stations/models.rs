use crate::shared::theme::{GREEN, MUTED, RED, YELLOW};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StationStatus {
    Available,
    InSession,
    Locked,
    Offline,
    Maintenance,
}

impl StationStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Available => "AVAILABLE",
            Self::InSession => "IN SESSION",
            Self::Locked => "LOCKED",
            Self::Offline => "OFFLINE",
            Self::Maintenance => "MAINTENANCE",
        }
    }

    pub fn color(self) -> u32 {
        match self {
            Self::Available => GREEN,
            Self::InSession => YELLOW,
            Self::Locked => RED,
            Self::Offline => MUTED,
            Self::Maintenance => YELLOW,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Station {
    pub id: String,
    pub status: StationStatus,
    pub customer: String,
    pub cpu: u8,
    pub gpu: u8,
    pub ram: u8,
    pub temp: u8,
}

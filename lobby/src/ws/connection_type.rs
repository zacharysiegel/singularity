use std::fmt::{Display, Formatter};

pub enum ConnectionType {
    Live,
    Lobby,
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ConnectionType::Live => "live",
            ConnectionType::Lobby => "lobby",
        };
        write!(f, "{}", string)
    }
}

use std::fmt::Display;

// TODO: Anything involving Order is just horrible code. Refactor.
pub enum Order {
    Shuffle,
    Album,
    Artist,
    Track,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Shuffle => write!(f, "Shuffle"),
            Order::Album => write!(f, "Album"),
            Order::Artist => write!(f, "Artist"),
            Order::Track => write!(f, "Track"),
        }
    }
}

impl Iterator for Order {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Order::Shuffle => Some(Order::Album),
            Order::Album => Some(Order::Artist),
            Order::Artist => Some(Order::Track),
            Order::Track => Some(Order::Shuffle),
        }
    }
}

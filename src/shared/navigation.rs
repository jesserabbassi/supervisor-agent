#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Dashboard,
    Stations,
    Sessions,
    Reservations,
    Customers,
    Wallet,
    Monitoring,
    Alerts,
    Reports,
    Settings,
}

impl Page {
    pub const ALL: [Page; 10] = [
        Self::Dashboard,
        Self::Stations,
        Self::Sessions,
        Self::Reservations,
        Self::Customers,
        Self::Wallet,
        Self::Monitoring,
        Self::Alerts,
        Self::Reports,
        Self::Settings,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Stations => "Gaming Stations",
            Self::Sessions => "Sessions",
            Self::Reservations => "Reservations",
            Self::Customers => "Customers",
            Self::Wallet => "Wallet / Payments",
            Self::Monitoring => "Monitoring",
            Self::Alerts => "Alerts",
            Self::Reports => "Reports",
            Self::Settings => "Settings",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Self::Dashboard => "⌂",
            Self::Stations => "▣",
            Self::Sessions => "◷",
            Self::Reservations => "▦",
            Self::Customers => "♙",
            Self::Wallet => "▤",
            Self::Monitoring => "◎",
            Self::Alerts => "⚠",
            Self::Reports => "▥",
            Self::Settings => "⚙",
        }
    }
    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Dashboard => "Here's what's happening at your gaming house today.",
            Self::Stations => "Manage and monitor all gaming PCs in real time",
            Self::Sessions => "Monitor and control all active gaming sessions",
            Self::Reservations => "Manage customer reservations and bookings",
            Self::Customers => "Manage your gaming house customers",
            Self::Wallet => "Manage customer wallets, payments and transactions",
            Self::Monitoring => "Real-time overview of all systems and gaming stations",
            Self::Alerts => "Stay informed about important events and system notifications",
            Self::Reports => "Gain insights into your gaming house performance",
            Self::Settings => "Manage your account, preferences and system configuration",
        }
    }
}

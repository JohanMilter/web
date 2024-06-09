mod drivers;
pub use drivers::{ChromeDriver, BraveDriver, EdgeDriver};

mod tabs;
pub use tabs::{ChromeTab, BraveTab, EdgeTab};

mod settings;
pub use settings::Settings;
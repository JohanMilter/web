mod drivers;
pub use drivers::{ChromeDriver, BraveDriver, EdgeDriver};

mod tabs;
pub use tabs::{ChromeTab, BraveTab, EdgeTab};

mod url;
pub use url::{ChromeUrl, BraveUrl, EdgeUrl};

mod element;
pub use element::{ChromeElement, BraveElement, EdgeElement};

mod settings;
pub use settings::Settings;
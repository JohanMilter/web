macros::import!(components > pub [*]);
macros::import!(logic > pub [*]);

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test()
    {
        let browser = browser::ChromeDriver::open(&browser::Settings {});
        let tab = browser.new_tab();
        let url = tab.navigate_url(UrlFrom::Url(""));
        let element = url.get_element(ElementFrom::Id(""));
        let attributes = element.read();
    }

    #[test]
    fn generate_dir_tree() {}
}


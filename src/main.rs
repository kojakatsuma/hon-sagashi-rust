use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use std::fs;

fn browse_wiki() -> Result<(), failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));
    let _jpg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &_jpg_data)?;
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    fs::write("screenshot.png", &_png_data)?;
    Ok(())
}

fn main() {
   assert!(browse_wiki().is_ok())
}

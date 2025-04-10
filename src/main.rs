mod args;

use std::sync::Arc;

use args::Args;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{Browser, Tab};
use path::Path;
use std::{fs, path};

const SCREENSHOTS_PATH: &str = "./screenshots";

fn create_browser_instance() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn create_browser_page(browser_instance: &Browser, base_url: &str) -> Arc<Tab> {
    let browser_tab = browser_instance.new_tab().unwrap();

    browser_tab.navigate_to(base_url).unwrap();

    browser_tab.wait_until_navigated().unwrap();

    browser_tab
}

fn extract_urls(browser_tab: &Arc<Tab>) -> Vec<String> {
    println!("Extracting urls");

    let site_map_page_link = browser_tab
        .find_elements(".sitemapPageLink")
        .unwrap_or_default()
        .iter()
        .filter_map(|element| element.get_attribute_value("nodeurl").unwrap())
        .collect::<Vec<String>>();

    println!("I've found these urls: {:?}", site_map_page_link);

    site_map_page_link
}

fn create_screenshots_directory() {
    println!("Creating screenshots folder {}", SCREENSHOTS_PATH);
    fs::remove_dir_all(SCREENSHOTS_PATH).unwrap();
    fs::create_dir(SCREENSHOTS_PATH).unwrap();
}

fn take_screenshot(browser_tab: &Arc<Tab>, page_url: String, index: usize, args: &Args) {
    let page_to_go = format!("{}/{}", args.base_url, page_url);
    println!("Taking screenshot {} for page {}", index, page_to_go);
    browser_tab.navigate_to(&page_to_go).unwrap();
    browser_tab.wait_until_navigated().unwrap();
    let screenshot_name = format!("{} - {}.png", index, page_url.replace(".html", ""));
    let png_data = browser_tab
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(100), None, true)
        .unwrap();

    fs::write(Path::new(SCREENSHOTS_PATH).join(screenshot_name), png_data).unwrap();
}

fn main() {
    let args = Args::from_args();
    println!("Backup of {}", args.base_url);
    let browser_instance = create_browser_instance();
    let browser_tab = create_browser_page(&browser_instance, &args.base_url);
    let urls = extract_urls(&browser_tab);
    create_screenshots_directory();
    for (index, url) in urls.iter().enumerate() {
        take_screenshot(&browser_tab, url.to_string(), index, &args);
    }

    browser_tab.close(false).unwrap();
}

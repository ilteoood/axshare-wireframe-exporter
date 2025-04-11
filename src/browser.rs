use std::sync::Arc;

use headless_chrome::{
    protocol::cdp::Page::{self, CaptureScreenshotFormatOption},
    Browser, Element, LaunchOptionsBuilder, Tab,
};

pub fn create_instance() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap()
}

pub fn create_page(browser_instance: &Browser, base_url: &str) -> Arc<Tab> {
    let browser_tab = browser_instance.new_tab().unwrap();

    browser_tab.navigate_to(base_url).unwrap();

    browser_tab.wait_until_navigated().unwrap();

    browser_tab
}

fn retrieve_page_value(html: &Element, function: &str) -> u32 {
    let page_value = html
        .call_js_fn(function, vec![], false)
        .unwrap()
        .value
        .unwrap()
        .as_u64()
        .unwrap();

    page_value as u32
}

pub fn retrieve_page_height(html: &Element) -> u32 {
    retrieve_page_value(
        html,
        r#"
        function getHeight() {
            return this.scrollHeight;
        }
    "#,
    )
}

pub fn retrieve_page_width(html: &Element) -> u32 {
    retrieve_page_value(
        html,
        r#"
        function getWidth() {
            return this.scrollWidth;
        }
    "#,
    )
}

pub fn go_to(browser_tab: &Arc<Tab>, url: &str) {
    browser_tab.navigate_to(&url).unwrap();
    browser_tab.wait_until_navigated().unwrap();
}

pub fn capture_full_width_screenshot(browser_tab: &Arc<Tab>) -> Vec<u8> {
    let html = browser_tab.wait_for_element("html").unwrap();

    browser_tab
        .call_method(Page::SetDeviceMetricsOverride {
            width: retrieve_page_width(&html),
            height: retrieve_page_height(&html),
            device_scale_factor: 1.0,
            mobile: false,
            position_x: Some(0),
            position_y: Some(0),
            scale: Some(1.0),
            screen_width: None,
            screen_height: None,
            dont_set_visible_size: Some(false),
            screen_orientation: None,
            viewport: None,
        })
        .unwrap();

    let png_data = browser_tab
        .capture_screenshot(CaptureScreenshotFormatOption::Jpeg, Some(100), None, true)
        .unwrap();

    png_data
}

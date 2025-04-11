use std::sync::Arc;

use headless_chrome::{Browser, Element, LaunchOptionsBuilder, Tab};

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

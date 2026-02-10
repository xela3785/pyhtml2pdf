use headless_chrome::{Browser, LaunchOptions, Tab};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use crate::error::PdfError;

// Singleton browser instance
static BROWSER: Lazy<Mutex<Option<Arc<Browser>>>> = Lazy::new(|| Mutex::new(None));

// Tab Pool
static TAB_POOL: Lazy<Mutex<Vec<Arc<Tab>>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn get_browser() -> Result<Arc<Browser>, PdfError> {
    let mut browser_guard = BROWSER.lock()
        .map_err(|_| PdfError::BrowserError("Failed to lock browser mutex".to_string()))?;

    if let Some(browser) = browser_guard.as_ref() {
        return Ok(browser.clone());
    }
    
    let args = vec![
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-dev-shm-usage",
    ];

    let launch_options = LaunchOptions {
        headless: true,
        sandbox: false,
        enable_gpu: false,
        enable_logging: false,
        idle_browser_timeout: std::time::Duration::from_secs(600),
        args: args.iter().map(|s| std::ffi::OsStr::new(s)).collect(),
        ..Default::default()
    };

    let browser = Browser::new(launch_options)
        .map_err(|e| PdfError::BrowserError(e.to_string()))?;

    let browser = Arc::new(browser);
    *browser_guard = Some(browser.clone());

    Ok(browser)
}

pub fn get_pooled_tab() -> Result<Arc<Tab>, PdfError> {
    // Try to get from poll
    {
        let mut pool = TAB_POOL.lock().unwrap();
        if let Some(tab) = pool.pop() {
            return Ok(tab);
        }
    }

    // If pool empty, create new
    let browser = get_browser()?;
    match browser.new_tab() {
        Ok(tab) => Ok(tab),
        Err(e) => {
            eprintln!("Failed to create new tab: {}. Restarting browser...", e);
            
            {
                let mut browser_guard = BROWSER.lock().unwrap();
                if let Some(current_browser) = browser_guard.as_ref() {
                    // Check if the browser we tried to use is still the current one
                    if Arc::ptr_eq(&browser, current_browser) {
                        *browser_guard = None;
                        // Clear the pool as well since it contains tabs from the dead browser
                        let mut pool = TAB_POOL.lock().unwrap();
                        pool.clear();
                    }
                }
            }

            // Get a fresh browser and try again
            let browser = get_browser()?;
            browser.new_tab()
                .map_err(|e| PdfError::BrowserError(format!("Failed to create new tab after restart: {}", e)))
        }
    }
}

pub fn recycle_tab(tab: Arc<Tab>) {
    let mut pool = TAB_POOL.lock().unwrap();
    pool.push(tab);
}
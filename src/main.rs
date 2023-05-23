use std::fs;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use headless_chrome::Browser;

use anyhow::Result;

fn main() -> Result<()> {
    let html_path = "test.html";
    let pdf_path = "test.pdf";

    let path = env::current_dir()?;

    let mut file = File::create(html_path)?;
    file.write_all(b"<h1>Testing</h1>")?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let url = format!("file://{}/{}", path.display(), html_path);
    let pdf = tab
        .navigate_to(&url)?
        .wait_until_navigated()?
        .print_to_pdf(None)?;
    
    fs::write(pdf_path, pdf)?;
    fs::remove_file(html_path)?;

    Ok(())
}

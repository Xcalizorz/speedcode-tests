#![feature(try_blocks)]

use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    // NOTE: For selenium 3.x, use "http://localhost:4444/wd/hub/session".
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver
        .goto("https://impressum.speed.codes/Privatperson")
        .await?;

    // Find element from element.
    let result: color_eyre::Result<()> = try {
        // Click Next.
        let mut elem_button = driver.find(By::LinkText("NEXT")).await?;
        elem_button.click().await?;

        let mut text_field = driver.find(By::Name("Vorname")).await?;
        text_field.wait_until().displayed().await?;
        text_field.send_keys("Abu").await?;

        text_field = driver.find(By::Name("Nachname")).await?;
        text_field.send_keys("Selenium").await?;

        text_field = driver.find(By::Name("Straße, Nr.")).await?;
        text_field.send_keys("Selenium 1").await?;

        text_field = driver.find(By::Name("Adresszusatz")).await?;
        text_field.send_keys("Erdgeschoss").await?;

        text_field = driver.find(By::Name("Postleitzahl")).await?;
        text_field.send_keys("22222").await?;

        text_field = driver.find(By::Name("Ort")).await?;
        text_field.send_keys("Hamburg").await?;

        text_field = driver.find(By::Name("Email")).await?;
        text_field.send_keys("selenium@test.de").await?;

        text_field = driver.find(By::Name("Telefon")).await?;
        text_field.send_keys("040 123123123").await?;

        // Click Next.
        elem_button = driver.find(By::LinkText("NEXT")).await?;
        elem_button.click().await?;

        text_field = driver
            .find(By::Name("Verantwortlich für den Inhalt"))
            .await?;
        text_field.send_keys("Abu Selenium").await?;

        // Look for header to implicitly wait for the page to load.
        println!("{}", driver.title().await?);

        // Click Submit.
        elem_button = driver.find(By::LinkText("SUBMIT")).await?;
        elem_button.click().await?;
    };

    println!("Got the following result: {:?}", result);

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;

    Ok(())
}

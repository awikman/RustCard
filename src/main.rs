use fantoccini::{Client, Locator};
use webdriver::capabilities::Capabilities;
use serde_json;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Error: URL argument is mandatory");
        process::exit(1);
    }


    let url = &args[1];

    let mut capabilities = Capabilities::new();
    let arg = serde_json::from_str("{\"args\": [\"-headless\"]}").unwrap();

    // If you don't want it headless, use this arg list instead:
    //let arg = serde_json::from_str("{\"args\": []}").unwrap();

    capabilities.insert("moz:firefoxOptions".to_string(), arg);

    // expects WebDriver instance to be listening at port 4444
    let mut client = Client::with_capabilities("http://localhost:4444", capabilities)
        .await
        .expect("failed to connect to WebDriver");


    // This of course need to be taken as a parameter...
    //let url = "https://www.kravattikaulaan.fi/";
    println!("Go to url: {}", url);
    // If this crashes, BardUrl or something, we never close the client, which leaves
    // the connection open. TODO...
    client.goto(url).await?;

    //TODO: This selector probably doesn't work for most shops...
    //let el_logo = client.find(Locator::Css("div.Header img").await?;


    let sel_logo = r#"div.Header img"#;
    let mut el_logo = client.find(Locator::Css(sel_logo)).await?;
    let logo_src = el_logo.attr("src").await?;
    println!("logo src: {}{}", url, logo_src.unwrap());

    let logo_width = el_logo.css("width").await?;
    println!("logo width: {}", logo_width.unwrap());

    let sel_body = r#"body"#;
    let mut el_body = client.find(Locator::Css(sel_body)).await?;
    let body_background_color = el_body.css("background-color").await?;
    println!("body background color: {}", body_background_color.unwrap());


    let sel_button = r#"button.AddToBasketButton"#;
    let mut el_button = client.find(Locator::Css(sel_button)).await?;
    let button_background = el_button.css("background-image").await?;
    println!("color theme: {}", button_background.unwrap());

    // Now I need to get stuff like this working:
    // window.getComputedStyle(document.querySelector('body')).color

    client.close().await?;

    Ok(())
}


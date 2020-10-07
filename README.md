# RustCard

Simple service that fetches a design elements from a site and automatically designs a
custom gift cards.

There are 3 parts of the system: 
- Headless browser with webdriver support (Tested with geckodriver)
- API that integrates uses the webdriver (Currently it's just a cli app)
- web client (currently there's just a static design demo, ´demo.html`)

For a simple demo of what kind of gift card designs we'd do, see [the demo](https://awikman.github.io/RustCard/demo.html)

For a live demo of fetching the data from a site, first run a webdriver on port `4444`, and run the cargo module with the URL as parameter.
Example: `cargo run https://www.kravattikaulaan.fi`

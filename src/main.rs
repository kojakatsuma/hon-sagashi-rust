use fantoccini::{ClientBuilder, Locator};


fn create_cap() -> serde_json::map::Map<String, serde_json::Value> {
    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({"args":["--headless"]});
    caps.insert("goog:chromeOptions".to_string(), opts.clone());
    caps
}

async fn get_items() -> Result<(), fantoccini::error::CmdError> {
    let caps = create_cap();
    let mut c = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");
    c.goto("https://www.amazon.co.jp/hz/wishlist/ls/1LT97CIJHMD3V?viewType=grid")
        .await?;
    const SCROLL: &str = r#"
        let end = document.getElementById('endOfListMarker')
        let count = 1
        let id = setInterval(()=> {
            window.scrollTo(0,10000*count)
            end = document.getElementById('endOfListMarker')
            console.log(document.querySelectorAll('div.a-section.a-spacing-none.wl-grid-item-content.wl-grid-item-flex-container > div > a').length)
            if(end){
                console.log("------------scroll end------------")
                clearInterval(id)
            }
            count ++
        },500)
    "#;
    c.execute(SCROLL, vec![]).await?;
    c.wait_for_find(Locator::Id("endOfListMarker")).await?;
    const GET_ITEMS: &str = r#"
        const [callback] = arguments;
        const titles = []
        document.querySelectorAll('div.a-section.a-spacing-none.wl-grid-item-content.wl-grid-item-flex-container > div > a')
            .forEach(({ title, href }) => title && href && titles.push({ title, amazonUrl: href }));
        callback(titles)
    "#;
    let items = c.execute_async(GET_ITEMS, vec![]).await?;
    let x = items.as_array().unwrap();
    print!("{}", x.len());
    for i in x {
        println!("{:?}", i);
    }
    c.close().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    get_items().await
}

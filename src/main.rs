use fantoccini::{error::CmdError, Client, ClientBuilder, Locator};
use serde_json::Value;
use tokio::process::Command;

fn create_cap() -> serde_json::map::Map<String, serde_json::Value> {
    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({"args":["--headless"]});
    caps.insert("goog:chromeOptions".to_string(), opts.clone());
    caps
}

async fn boot_chromedriver() {
    Command::new("chromedriver")
        .spawn()
        .expect("chromedriver command failed to run")
        .wait()
        .await
        .expect("chromedriver command failed to run");
}

async fn get_items(mut c: Client) -> Result<Vec<Value>, CmdError> {
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
    let items = c
        .execute_async(GET_ITEMS, vec![])
        .await?
        .as_array()
        .unwrap()
        .clone();
    Ok(items)
}

async fn search_lib(mut c: Client) -> Result<(), CmdError> {
    c.goto("https://www.lib.city.ota.tokyo.jp/index.html")
        .await?;
    c.wait_for_find(Locator::Css(".imeon")).await?;
    const TYPE_TITLE: &str = r#"
        const [callback] = arguments;
        document.querySelector(".imeon").value = "日本大衆文化史";
        callback();
    "#;
    c.execute_async(TYPE_TITLE, vec![]).await?;
    c.find(Locator::Css("input[name='buttonSubmit']")).await?.click().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // boot_chromedriver().await;
    let mut c = ClientBuilder::native()
        // .capabilities(create_cap())
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");
    let items = get_items(c.clone()).await.expect("error");
    search_lib(c.clone()).await?;
    c.close().await
}

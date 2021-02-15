use fantoccini::ClientBuilder;

async fn get_items() -> Result<(), fantoccini::error::CmdError> {
    // let cap = webdriver::capabilities::Capabilities::new();
    let mut c = ClientBuilder::native()
        // .capabilities()
        .connect("http://localhost:9515")
        .await
        .expect("failed to connect to WebDriver");
    c.goto("https://www.amazon.co.jp/hz/wishlist/ls/1LT97CIJHMD3V?viewType=grid")
        .await?;
    const SCROLL: &str = r#"
        let end = document.getElementById('endOfListMarker')
        let count = 1;
        let id = setInterval(()=> {
            window.scrollTo(0,window.outerHeight*count)
            end = document.getElementById('endOfListMarker')
            count ++   
        },500)
    "#;
    c.execute(SCROLL, vec![]).await?;
    const GET_ITEMS: &str = r#"
        const [callback] = arguments;
        let end = document.getElementById('endOfListMarker')
        let id = setInterval(() => {
            if(end){
                const titles = []
                document.querySelectorAll('div.a-section.a-spacing-none.wl-grid-item-content.wl-grid-item-flex-container > div > a')
                    .forEach(({ title, href }) => title && href && titles.push({ title, amazonUrl: href }));
                callback(titles)
            }
            end = document.getElementById('endOfListMarker')
        },500)
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

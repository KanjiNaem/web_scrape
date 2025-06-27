use reqwest::Client;
use scraper::html;



#[tokio::main]
async fn main() {

    let client = Client::new();
    let response = client.get("https://scrapeme.live/shop/")
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();
    let document  = scraper::Html::parse_document(&html_content); 
    let product_selector = scraper::Selector::parse("li.product")
    .unwrap();

    let html_products = document.select(&product_selector);
    for product in html_products {
        
        let url = product.select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href")).map(str::to_owned);

        let img_url = product.select(&scraper::Selector::parse("img").unwrap())
        .next()
        .and_then(|img| img.value().attr("src")).map(str::to_owned);
    
        let product_name = product.select(&scraper::Selector::parse("h2").unwrap())
        .next()
        .map(|h2| h2.text().collect::<String>());
    
        let price = product.select(&scraper::Selector::parse(".price").unwrap())
        .next()
        .map(|h2| h2.text().collect::<String>());

        println!("-----------------------------");
        println!("name = {:?}, price = {:?}, url = {:?}, img_url = {:?}, ", product_name, price, url, img_url);
        println!("-----------------------------");
    }
}

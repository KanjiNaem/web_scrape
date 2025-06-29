// use reqwest::Client;

// #[tokio::main]
// async fn main() {

//     let client = Client::new();
//     let response = client.get("https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=false")
//         .send()
//         .await
//         .unwrap();

//     let html_content = response.text().await.unwrap();
//     let doc  = scraper::Html::parse_document(&html_content);
//     let prop_selector = scraper::Selector::parse("div.filter-results-holder")
//     .unwrap();

//     let html_item = doc.select(&prop_selector);
//     for item in html_item {
//         let url = item.select(&scraper::Selector::parse("a").unwrap())
//             .next()
//             .and_then(|a| a.value().attr("href")).map(str::to_owned);
//         println!("{:?}", url);
//     }

// }

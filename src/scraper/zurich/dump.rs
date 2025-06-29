//     for product in html_products {

//         let url = product.select(&scraper::Selector::parse("a").unwrap())
//         .next()
//         .and_then(|a| a.value().attr("href")).map(str::to_owned);

//         let img_url = product.select(&scraper::Selector::parse("img").unwrap())
//         .next()
//         .and_then(|img| img.value().attr("src")).map(str::to_owned);

//         let product_name = product.select(&scraper::Selector::parse("h2").unwrap())
//         .next()
//         .map(|h2| h2.text().collect::<String>());

//         let price = product.select(&scraper::Selector::parse(".price").unwrap())
//         .next()
//         .map(|h2| h2.text().collect::<String>());

//    ;
// }

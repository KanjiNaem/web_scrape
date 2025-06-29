use reqwest::Client;
use scraper::Selector;
use smart_default::SmartDefault;

async fn get_page_amount(url_str: String) -> i8 {
    let client = Client::new();
    let response = client.get(url_str).send().await.unwrap();

    let html_content = response.text().await.unwrap();
    let doc = scraper::Html::parse_document(&html_content);
    let page_selector: Selector = scraper::Selector::parse("ul.pages>li").unwrap();
    let html_item = doc.select(&page_selector);

    let mut max_page: i8 = 0;
    for item in html_item {
        let test = item
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|page_num| page_num.value().attr("data-page-index"))
            .map(str::to_owned);

        match test {
            Some(num_str) => {
                let extracted_num: i8 = num_str.parse().expect("Failed to parse str to int");
                if extracted_num > max_page {
                    max_page = extracted_num;
                }
            }
            None => {}
        }
    }

    return max_page;
}

async fn get_prop_url(url_str: String) -> String {
    let client = Client::new();
    let response = client.get(url_str).send().await.unwrap();

    let html_content = response.text().await.unwrap();
    let doc = scraper::Html::parse_document(&html_content);
    let prop_selector = scraper::Selector::parse("div.filter-item").unwrap();

    // println!("{:?}", prop_selector);

    let html_item = doc.select(&prop_selector);
    let mut str_result: String = "".to_owned();

    for item in html_item {
        let url_start = "https://www.immobilier.ch";
        let url_end = item
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let url = url_start.to_owned() + &url_end.unwrap_or_default();
        str_result = format!("{str_result}{url}\n");
    }

    return str_result;
}

#[derive(SmartDefault, Debug)]
struct UrlParams {
    #[default((0, 0))]
    rent_amount: (i32, i32),
    #[default((0.0, 0.0))]
    room_amount: (f32, f32),
    #[default((0, 0))]
    surface: (i32, i32),
    #[default("zurich".to_owned())]
    area: String,
    #[default(vec!["".to_owned()])]
    keywords: Vec<String>,
    #[default(false)]
    new_obj: bool,
}

async fn gen_url_base(params: &UrlParams) -> String {
    let mut extr_rent_amount: String = "".to_owned();
    let mut extr_room_amount: String = "".to_owned();
    let mut extr_surface: String = "".to_owned();
    let mut extr_area: String = "zurich".to_owned();
    let mut extr_keywords: String = "".to_owned();
    let mut extr_new_obj: String = "&nb=false".to_owned();

    match params.rent_amount {
        (0, 0) => {}
        _ => {
            let rent_str_head: String =
                "&pn=".to_owned() + &params.rent_amount.0.to_string().to_owned();
            let rent_str_tail: String =
                "&px=".to_owned() + &params.rent_amount.1.to_string().to_owned();

            extr_rent_amount = if &params.rent_amount.0 == &0 {
                rent_str_tail
            } else if &params.rent_amount.1 == &0 {
                rent_str_head
            } else {
                rent_str_head + rent_str_tail.as_str()
            }
        }
    }

    match params.room_amount {
        (0.0, 0.0) => {}
        _ => {
            let room_str_head: String =
                "&nrn=".to_owned() + &params.room_amount.0.to_string().to_owned();
            let room_str_tail: String =
                "&nrx=".to_owned() + &params.room_amount.1.to_string().to_owned();

            extr_room_amount = if &params.room_amount.0 == &0.0 {
                room_str_tail
            } else if &params.room_amount.1 == &0.0 {
                room_str_head
            } else {
                room_str_head + room_str_tail.as_str()
            }
        }
    }

    match params.surface {
        (0, 0) => {}
        _ => {
            let surface_head: String = "&sn=".to_owned() + &params.surface.0.to_string().to_owned();
            let surface_tail: String = "&sx=".to_owned() + &params.surface.1.to_string().to_owned();

            extr_surface = if &params.surface.0 == &0 {
                surface_tail
            } else if &params.surface.1 == &0 {
                surface_head
            } else {
                surface_head + surface_tail.as_str()
            }
        }
    }

    match &params.area {
        area if area == &"zurich".to_owned() => {}
        _ => {
            extr_area = params.area.clone();
        }
    }

    match &params.keywords {
        keyword_vec if keyword_vec == &vec!["".to_owned()] => {}
        _ => {
            extr_keywords = "&k=".to_owned();
            for curr_keyword in &params.keywords {
                extr_keywords = (extr_keywords + curr_keyword) + ";";
            }
            extr_keywords.truncate(extr_keywords.len() - 1);
        }
    }
    match &params.new_obj {
        new_obj if new_obj == &false => {}
        _ => extr_new_obj = "&nb=true".to_owned(),
    }

    let mut url_result: String = "https://www.immobilier.ch/en/rent/apartment/".to_owned()
        + extr_area.as_str()
        + "/page-1?t=rent&c=1&p=s126"; // for zurich, others not implemented yet
    url_result = format!(
        "{}{}{}{}{}{}",
        url_result, extr_rent_amount, extr_room_amount, extr_surface, extr_keywords, extr_new_obj
    );

    return url_result;
}

#[tokio::main]
async fn main() {
    let mut test_params = UrlParams::default();
    // default test params
    // println!("{:#?}", test_params);

    // modified test params
    // test_params.rent_amount = (400, 800);
    // test_params.room_amount = (1.5, 4.5);
    // test_params.surface = (50, 0);
    // test_params.area = "zurich".to_owned();
    // test_params.keywords = vec!["AAAAA".to_owned()];
    // test_params.new_obj = false;
    // println!("{:#?}", test_params);

    let test_url = gen_url_base(&test_params).await;
    // println!("{}", test_url);
    // https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=false
    // https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=false
    // https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&pn=400&px=800&nrn=1.5&nrx=4.5&sn=50&k=AAAAA&nb=false

    let page_amount = get_page_amount(test_url.clone()).await;
    println!("{}", page_amount);
    let prop_url = get_prop_url(test_url.clone()).await;
    println!("{}", prop_url);
}

// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=false base
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&pn=600&px=800&nb=false  rent_amount
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nrn=1&nrx=4&nb=false    room_amount
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&sn=10&sx=60&nb=false    surface
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=false&    area
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&k=XXXXXXX;YYYYYYY;ZZZZZZZ&nb=false keywords
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&nb=true new_obj
// https://www.immobilier.ch/en/rent/apartment/zurich/page-1?t=rent&c=1&p=s126&pn=200&px=1800&nrn=1&nrx=4.5&sn=10&sx=90&k=XXXX;YYYY&nb=true

// default out:
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/homenhancement-2562/modern-1-bedroom-apartment-for-rent-in-zurich-ideal-location-fully-renovated-1259425
// https://www.immobilier.ch/en/rent/apartment/zurich/bulach/betterhomes-schweiz-ag-611/bright-and-modern-1319959
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/wincasa-siege-602/central-living-in-horgen-oberdorf-with-view-of-lake-zurich-1318850
// https://www.immobilier.ch/en/rent/apartment/zurich/zollikon/schaeppi-grundstucke-ag-808/charming-maisonette-apartment-in-zollikon-for-rent-1318104
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/wohnplus-ag-2710/modern-apartment-in-top-location-1318556
// https://www.immobilier.ch/en/rent/apartment/zurich/adliswil/caisse-pensions-migros-1542/great-apartment-with-charm-right-on-the-sihl-1318474
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/livit-ag-real-estate-management-675/spacious-apartment-in-central-location-1320343
// https://www.immobilier.ch/en/rent/apartment/zurich/fallanden/wincasa-siege-602/charming-property-near-greifensee-1319638
// https://www.immobilier.ch/en/rent/apartment/zurich/stallikon/privera-ag-gumlingen-969/cozy-45-room-apartment-1317372
// https://www.immobilier.ch/en/rent/apartment/zurich/kilchberg-zh/wincasa-siege-602/living-experience-on-lake-zurich-1318618
// https://www.immobilier.ch/en/rent/apartment/zurich/kusnacht-zh/wincasa-siege-602/your-new-home-in-prime-location-in-kusnacht-1318447
// https://www.immobilier.ch/en/rent/apartment/zurich/kloten/schaeppi-grundstucke-ag-808/fantastic-45-room-new-apartment-1318105
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/apleona-schweiz-ag-670/charming-25-room-apartment-in-green-oasis-limited-until-30092027-1317263
// https://www.immobilier.ch/en/rent/apartment/zurich/regensdorf/livit-ag-real-estate-management-675/penthouse-apartment-in-regensdorf-1317318
// https://www.immobilier.ch/en/rent/apartment/zurich/oberglatt-zh/livit-ag-real-estate-management-675/high-living-comfort-in-family-friendly-environment-1318030
// https://www.immobilier.ch/en/rent/apartment/zurich/erlenbach-zh/caisse-pensions-migros-1542/charming-apartment-with-seating-area-limited-until-30062026-1318475
// https://www.immobilier.ch/en/rent/apartment/zurich/aesch-zh/properti-ag-2229/modernized-15-room-apartment-in-aesch-1317113
// https://www.immobilier.ch/en/rent/apartment/zurich/volketswil/apleona-schweiz-ag-670/45-room-apartment-in-quiet-development-1318626
// https://www.immobilier.ch/en/rent/apartment/zurich/oberrieden/apleona-schweiz-ag-670/live-near-lake-zurich-1318633
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/livit-ag-real-estate-management-675/city-proximity-amp-comfort-living-with-style-1318722
// https://www.immobilier.ch/en/rent/apartment/zurich/adliswil/schaeppi-grundstucke-ag-808/spacious-45-room-apartment-in-green-adliswil-for-rent-1318106
// https://www.immobilier.ch/en/rent/apartment/zurich/pfaffikon-zh/wincasa-siege-602/rent-without-deposit-beautiful-apartment-in-green-surroundings-1318031
// https://www.immobilier.ch/en/rent/apartment/zurich/fahrweid/privera-ag-gumlingen-969/beautiful-apartment-with-cozy-loggia-1319770

// modified test out:
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/homenhancement-2562/modern-1-bedroom-apartment-for-rent-in-zurich-ideal-location-fully-renovated-1259425
// https://www.immobilier.ch/en/rent/apartment/zurich/schwerzenbach/wincasa-siege-602/looking-for-sunny-apartment-1318032
// https://www.immobilier.ch/en/rent/apartment/zurich/zollikon/schaeppi-grundstucke-ag-808/charming-maisonette-apartment-in-zollikon-for-rent-1318104
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/wincasa-siege-602/spacious-attic-apartment-rent-without-deposit-1319640
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/apleona-schweiz-ag-670/-1317266
// https://www.immobilier.ch/en/rent/apartment/zurich/adliswil/schaeppi-grundstucke-ag-808/spacious-45-room-apartment-in-green-adliswil-for-rent-1318106
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/wincasa-siege-602/central-living-in-horgen-oberdorf-with-view-of-lake-zurich-1318850
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/kornhaus-verwaltungs-ag-regie-immobiliere-475/between-the-limmat-and-the-vineyards-living-in-hongg-1318863
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/caisse-pensions-migros-1542/35-room-maisonette-apartment-near-the-lake-1319007
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/livit-ag-real-estate-management-675/we-are-renting-this-central-and-modern-apartment-in-altstetten-1319668
// https://www.immobilier.ch/en/rent/apartment/zurich/aesch-zh/properti-ag-2229/modernized-15-room-apartment-in-aesch-1317113
// https://www.immobilier.ch/en/rent/apartment/zurich/regensdorf/livit-ag-real-estate-management-675/penthouse-apartment-in-regensdorf-1317318
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/livit-ag-real-estate-management-675/ideal-single-apartment-with-optimal-transport-connections-1318027
// https://www.immobilier.ch/en/rent/apartment/zurich/seuzach/apleona-schweiz-ag-670/central-single-apartment-with-private-laundry-tower-1318627
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/livit-ag-real-estate-management-675/quietly-located-near-the-university-limited-for-one-year-1318723
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/wincasa-siege-602/modern-new-apartment-in-horgen-oberdorf-with-lake-view-1319635
// https://www.immobilier.ch/en/rent/apartment/zurich/adliswil/livit-ag-real-estate-management-675/beautiful-apartment-in-family-friendly-environment-1317080
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/properti-ag-2229/exclusive-pied-terre-office-with-spacious-terrace-in-prime-city-location-in-zurich-1318520
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/schaeppi-grundstucke-ag-808/central-oasis-in-district-2-your-new-city-apartment-awaits-1317861
// https://www.immobilier.ch/en/rent/apartment/zurich/adliswil/caisse-pensions-migros-1542/great-apartment-with-charm-right-on-the-sihl-1318474
// https://www.immobilier.ch/en/rent/apartment/zurich/kilchberg-zh/wincasa-siege-602/living-experience-on-lake-zurich-1318618
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/schaeppi-grundstucke-ag-808/35-room-apartment-at-sonnenbergstrasse-5-8032-zurich-1318786
// https://www.immobilier.ch/en/rent/apartment/zurich/fahrweid/privera-ag-gumlingen-969/beautiful-apartment-with-cozy-loggia-1319770

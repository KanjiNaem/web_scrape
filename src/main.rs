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

    let test_url = gen_url_base(&test_params).await;
    // println!("{}", test_url);
    let page_amount = get_page_amount(test_url.clone()).await;
    println!("{}", page_amount);
    let prop_url = get_prop_url(test_url.clone()).await;
    println!("{}", prop_url);

    // modified test params
    test_params.rent_amount = (400, 800);
    test_params.room_amount = (1.5, 4.5);
    test_params.surface = (50, 0);
    test_params.area = "zurich".to_owned();
    test_params.keywords = vec!["AAAAA".to_owned()];
    test_params.new_obj = false;
    // println!("{:#?}", test_params);

    let test_url = gen_url_base(&test_params).await;
    // println!("{}", test_url);
    let page_amount = get_page_amount(test_url.clone()).await;
    println!("{}", page_amount);
    let prop_url = get_prop_url(test_url.clone()).await;
    println!("{}", prop_url);
}

// default out:
// 25
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/homenhancement-2562/modern-1-bedroom-apartment-for-rent-in-zurich-ideal-location-fully-renovated-1259425
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/wincasa-siege-602/spacious-attic-apartment-rent-without-deposit-1319640
// https://www.immobilier.ch/en/rent/apartment/zurich/zumikon/kornhaus-verwaltungs-ag-regie-immobiliere-475/your-newly-renovated-home-1318862
// https://www.immobilier.ch/en/rent/apartment/zurich/hinteregg/betterhomes-schweiz-ag-611/holiday-feeling-on-3-floors-with-large-terrace-1317603
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/kornhaus-verwaltungs-ag-regie-immobiliere-475/between-the-limmat-and-the-vineyards-living-in-hongg-1318863
// https://www.immobilier.ch/en/rent/apartment/zurich/meilen/privera-ag-gumlingen-969/your-new-living-space-1318925
// https://www.immobilier.ch/en/rent/apartment/zurich/niederglatt-zh/properti-ag-2229/charming-35-room-apartment-in-central-location-in-niederglatt-canton-zurich-1317786
// https://www.immobilier.ch/en/rent/apartment/zurich/volketswil/apleona-schweiz-ag-670/45-room-apartment-in-quiet-development-1318626
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/livit-ag-real-estate-management-675/ideal-single-apartment-with-optimal-transport-connections-1318027
// https://www.immobilier.ch/en/rent/apartment/zurich/kusnacht-zh/wincasa-siege-602/your-new-home-in-prime-location-in-kusnacht-1318447
// https://www.immobilier.ch/en/rent/apartment/zurich/seuzach/apleona-schweiz-ag-670/central-single-apartment-with-private-laundry-tower-1318627
// https://www.immobilier.ch/en/rent/apartment/zurich/opfikon/betterhomes-schweiz-ag-611/luxurious-in-prime-location-1319108
// https://www.immobilier.ch/en/rent/apartment/zurich/uster/wincasa-siege-602/rent-without-deposit-apartment-in-central-location-1317849
// https://www.immobilier.ch/en/rent/apartment/zurich/stafa/betterhomes-schweiz-ag-611/neubau-ab-september-2025-1318449
// https://www.immobilier.ch/en/rent/apartment/zurich/erlenbach-zh/privera-ag-gumlingen-969/your-new-living-space-1318929
// https://www.immobilier.ch/en/rent/apartment/zurich/zollikerberg/properti-ag-2229/spacious-75-room-maisonette-apartment-with-views-at-zollikerberg-1320369
// https://www.immobilier.ch/en/rent/apartment/zurich/oberglatt-zh/livit-ag-real-estate-management-675/high-living-comfort-in-family-friendly-environment-1318030
// https://www.immobilier.ch/en/rent/apartment/zurich/fahrweid/privera-ag-gumlingen-969/beautiful-apartment-with-cozy-loggia-1319770
// https://www.immobilier.ch/en/rent/apartment/zurich/pfaffikon-zh/wincasa-siege-602/rent-without-deposit-beautiful-apartment-in-green-surroundings-1318031
// https://www.immobilier.ch/en/rent/apartment/zurich/stallikon/privera-ag-gumlingen-969/cozy-45-room-apartment-1317372
// https://www.immobilier.ch/en/rent/apartment/zurich/schwerzenbach/wincasa-siege-602/looking-for-sunny-apartment-1318032
// https://www.immobilier.ch/en/rent/apartment/zurich/oberrieden/apleona-schweiz-ag-670/live-near-lake-zurich-1318633
// https://www.immobilier.ch/en/rent/apartment/zurich/geroldswil/immosky-ag-1606/ideal-for-senior-living-safe-and-independent-living-1320553

// modified nonsense url out:
// 25
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/homenhancement-2562/modern-1-bedroom-apartment-for-rent-in-zurich-ideal-location-fully-renovated-1259425
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/livit-ag-real-estate-management-675/ideal-single-apartment-with-optimal-transport-connections-1318027
// https://www.immobilier.ch/en/rent/apartment/zurich/erlenbach-zh/privera-ag-gumlingen-969/your-new-living-space-1318929
// https://www.immobilier.ch/en/rent/apartment/zurich/stafa/betterhomes-schweiz-ag-611/new-building-from-september-2025-1318410
// https://www.immobilier.ch/en/rent/apartment/zurich/winterthur/wincasa-siege-602/spacious-attic-apartment-rent-without-deposit-1319640
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/kornhaus-verwaltungs-ag-regie-immobiliere-475/cozy-attic-apartment-in-the-heart-of-zurich-1319695
// https://www.immobilier.ch/en/rent/apartment/zurich/stallikon/privera-ag-gumlingen-969/cozy-45-room-apartment-1317372
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/properti-ag-2229/exclusive-pied-terre-office-with-spacious-terrace-in-prime-city-location-in-zurich-1318520
// https://www.immobilier.ch/en/rent/apartment/zurich/urdorf/livit-ag-real-estate-management-675/apartment-in-family-friendly-development-limited-until-31102027-1319422
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/livit-ag-real-estate-management-675/we-are-renting-this-central-and-modern-apartment-in-altstetten-1319668
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/apleona-schweiz-ag-670/charming-25-room-apartment-in-green-oasis-limited-until-30092027-1317263
// https://www.immobilier.ch/en/rent/apartment/zurich/zumikon/kornhaus-verwaltungs-ag-regie-immobiliere-475/your-newly-renovated-home-1318862
// https://www.immobilier.ch/en/rent/apartment/zurich/opfikon/betterhomes-schweiz-ag-611/luxurious-in-prime-location-1319108
// https://www.immobilier.ch/en/rent/apartment/zurich/aesch-zh/properti-ag-2229/modernized-15-room-apartment-in-aesch-1317113
// https://www.immobilier.ch/en/rent/apartment/zurich/regensdorf/livit-ag-real-estate-management-675/penthouse-apartment-in-regensdorf-1317318
// https://www.immobilier.ch/en/rent/apartment/zurich/stafa/betterhomes-schweiz-ag-611/new-building-from-september-2025-1318439
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/betterhomes-schweiz-ag-611/modern-architecture-in-the-green-1319095
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/kornhaus-verwaltungs-ag-regie-immobiliere-475/between-the-limmat-and-the-vineyards-living-in-hongg-1318863
// https://www.immobilier.ch/en/rent/apartment/zurich/zollikerberg/apleona-schweiz-ag-670/living-close-to-nature-not-far-from-the-city-1319615
// https://www.immobilier.ch/en/rent/apartment/zurich/oberglatt-zh/livit-ag-real-estate-management-675/high-living-comfort-in-family-friendly-environment-1318030
// https://www.immobilier.ch/en/rent/apartment/zurich/horgen/wincasa-siege-602/central-living-in-horgen-oberdorf-with-view-of-lake-zurich-1318850
// https://www.immobilier.ch/en/rent/apartment/zurich/zurich/wohnplus-ag-2710/top-renovated-15-room-apartment-1317142
// https://www.immobilier.ch/en/rent/apartment/zurich/kilchberg-zh/wincasa-siege-602/living-experience-on-lake-zurich-1318618

use reqwest::header::{CONTENT_TYPE, CONNECTION, ACCEPT_ENCODING};

fn rest_api() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("https://api.myip.com")?;
    let body: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
    println!("IP: {}", body["ip"]);
    Ok(())
}

fn soap_api() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let xml = r#"<?xml version="1.0" encoding="utf-8"?><soap12:Envelope xmlns:soap12="http://www.w3.org/2003/05/soap-envelope"><soap12:Body><ListOfCountryNamesByName xmlns="http://www.oorsprong.org/websamples.countryinfo"></ListOfCountryNamesByName></soap12:Body></soap12:Envelope>"#;

    let builder = client.post("http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso")
    .header(CONTENT_TYPE, "text/xml; charset=utf-8")
    .header(CONNECTION, "keep-alive")
    .header(ACCEPT_ENCODING, "gzip, deflate, br")
    .body(xml);

    let res = builder.send()?;
    let xml = res.text()?;
    let doc = roxmltree::Document::parse(&xml)?;
    println!("Here is an alphabetical list of all countries: ");
    for elem in doc.descendants() {
        if !elem.is_text() {
            continue;
        }
        match elem.text() {
            Some(text) => {
                if !text.contains("\n") && text.len() != 2 {
                    println!("{}, ", text);
                }
            },
            None => continue,
        }
    }

    Ok(())
}

fn main() {
    let _ = rest_api();
    let _ = soap_api();
}

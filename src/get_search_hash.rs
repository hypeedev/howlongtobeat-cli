use reqwest::Client;
use regex::Regex;

pub async fn get_search_hash(client: &Client) -> String {
    let html: String = client.get("https://howlongtobeat.com")
        .send().await.unwrap()
        .text().await.unwrap();

    let re = Regex::new(r#"/_next/static/chunks/pages/_app-[a-f0-9]{16}.js"#).unwrap();
    let app_script_path = re.find(&html).unwrap().as_str();

    let app_script: String = client.get(format!("https://howlongtobeat.com{}", app_script_path))
        .send().await.unwrap()
        .text().await.unwrap();

    let re = Regex::new(r#""/api/find/"\.concat\("([a-f0-9]+?)"\)\.concat\("([a-f0-9]+?)"\)"#).unwrap();
    let search_hash = re.captures(&app_script).unwrap().iter().skip(1).map(|x| x.unwrap().as_str()).collect::<Vec<&str>>().join("");

    search_hash.to_string()
}
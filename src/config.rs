use reqwasm::http::Request;
use serde::Deserialize;

// http://localhost:8080/config/dsiem_config.json

#[derive(Deserialize, Clone, PartialEq)]
pub struct DsiemConfig {
    pub status: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq, Default)]
pub struct SearchConfig {
    #[serde(rename(deserialize = "elasticsearch"))]
    pub search: String,
    #[serde(rename(deserialize = "kibana"))]
    pub dashboard: String,
}

pub async fn get_search_endpoints(dsiem_baseurl: String) -> Result<SearchConfig, String> {
    let url = dsiem_baseurl + "/ui/assets/config/esconfig.json";
    let resp = Request::get(&url)
        .send().await
        .map_err(|e| e.to_string())?;
    let body = resp.text().await.map_err(|e| e.to_string())?;
    let mut config: SearchConfig = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    if !config.search.ends_with('/') {
        config.search += "/";
    }
    if !config.dashboard.ends_with('/') {
        config.dashboard += "/";
    }
    Ok(config)
}
pub async fn read(dsiem_baseurl: String) -> Result<DsiemConfig, String> {
    let url = dsiem_baseurl + "/config/dsiem_config.json";
    let resp = Request::get(&url)
        .send().await
        .map_err(|e| e.to_string())?;
    let body = resp.text().await.map_err(|e| e.to_string())?;
    let config: DsiemConfig = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(config)
}
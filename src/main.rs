use base64::Engine;

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde_json::json;
use std::{env, io::Read};
use std::{fs::File, path::Path};

async fn access_token(drop: &DropboxService) -> String {
    let refresh_token = drop.refreshToken.clone();

    let b64 = base64::engine::general_purpose::STANDARD.encode(&format!(
        "{}:{}",
        drop.client.clone(),
        drop.appSercret.clone()
    ));

    let url = "https://api.dropbox.com/oauth2/token";
    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Basic {}", b64)).unwrap(),
    );

    let body = format!("grant_type=refresh_token&refresh_token={}", refresh_token);

    let client = Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .expect("Falhou");

    if !response.status().is_success() {
        let text = response.text().await.unwrap();

        panic!("{:?}", text);
    }

    let a = response.json::<serde_json::Value>().await.unwrap();

    let access_token = a["access_token"].as_str().unwrap();

    access_token.to_string()
}

#[tokio::main]
async fn main() {
    let drop = DropboxService {
        refreshToken: String::from(""),
        client: String::from(""),
        appKey: String::from(""),
        appSercret: String::from(""),
    };

    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap().to_string();
    // let path = String::from("sample.pdf");

    let token = access_token(&drop).await;

    drop.upload(path, token).await;
}

struct StatusResponse {
    pub message: Option<String>,
    pub success: Option<bool>,
}

impl StatusResponse {
    pub fn new() -> StatusResponse {
        StatusResponse {
            message: Some(String::from("")),
            success: Some(false),
        }
    }
}
struct DropboxService {
    pub refreshToken: String,
    pub client: String,
    pub appKey: String,
    pub appSercret: String,
}

impl DropboxService {
    pub async fn upload(self, file_path: String, access_token: String) -> StatusResponse {
        let path = Path::new(&file_path);
        let mut file = File::open(file_path.clone()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let url = "https://content.dropboxapi.com/2/files/upload";
        let mut headers = HeaderMap::new();

        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();

        let payload = json!({
            "autorename": true,
            "mode": "add",
            "mute": false,
            "path": format!("/{}", filename),
            "strict_conflict": false
        });

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );

        headers.insert(
            "Dropbox-API-Arg",
            HeaderValue::from_str(payload.to_string().as_str()).unwrap(),
        );

        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/octet-stream"),
        );

        let c = Client::new()
            .post(url)
            .headers(headers)
            .body(buffer)
            .send()
            .await
            .expect("Não subiu o arquivo");

        let mut status_response = StatusResponse::new();

        status_response.success = Some(c.status().is_success());
        status_response.message = Some(c.text().await.unwrap());

        return status_response;
    }
}

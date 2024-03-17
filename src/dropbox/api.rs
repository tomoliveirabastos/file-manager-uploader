pub mod api {
    pub async fn access_token(drop: &DropboxService) -> String {
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
    
}
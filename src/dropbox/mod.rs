mod status_message;

pub mod dropbox {
    use std::{fs::File, path::Path};

    use serde_json::json;
    use std::{env, io::Read};
    use reqwest::{
        header::{HeaderMap, HeaderValue, CONTENT_TYPE},
        Client,
    };
    
    use super::status_message::status_message::StatusResponse;

    pub struct DropboxService {
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
}

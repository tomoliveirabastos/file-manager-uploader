mod dropbox;

use base64::Engine;
use dropbox::dropbox::{DropboxService};
use dropbox::api::{access_token};

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};

use serde_json::json;
use std::{env, io::Read};
use std::{fs::File, path::Path};


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


pub mod status_message {
    pub struct StatusResponse {
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
}
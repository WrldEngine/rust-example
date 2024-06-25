use dotenv::dotenv;

pub struct Settings {
    pub user: String,
}

impl Settings {
    pub async fn get_env() -> Settings {
        dotenv().ok();
        let user: String = std::env::var("USER").expect("USER must be set.");
        Settings { user }
    }
}
use once_cell::sync::Lazy;
use reqwest::Client;

pub static HTTP: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("walli/0.1 (+https://github.com/sara/walli)")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("http client")
});

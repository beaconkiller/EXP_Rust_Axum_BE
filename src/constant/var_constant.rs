use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct StrConfig {
    pub port: i16,
    pub addr: String,
}

pub struct VarConstant;

impl VarConstant {
    pub fn get_config() -> StrConfig {
        dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse::<i16>()
            .unwrap();

        let addr: String = env::var("ADDR").unwrap_or("err".to_string());

        StrConfig {
            port: port,
            addr: addr,
        }
    }
}

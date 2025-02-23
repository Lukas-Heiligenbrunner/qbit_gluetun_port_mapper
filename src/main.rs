use dotenvy::dotenv;
use env_logger::Env;
use log::info;
use qbit_rs::model::Credential;
use qbit_rs::Qbit;
use result_logger::ResultLogger;
use std::fs;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    _ = dotenv();
    init_logger();

    let inverval = std::env::var("INTERVAL").unwrap_or("600".to_string());
    let interval = inverval
        .parse::<u64>()
        .expect("Failed to parse interval to i64");
    let mut interval = time::interval(Duration::from_secs(interval));

    let api = connect_qbit()
        .await
        .error()
        .expect("Failed to connect to qbittorrent");

    loop {
        interval.tick().await; // Wait for the interval
        let port = read_port().expect("Failed to read port from file");
        set_port(port, &api)
            .await
            .error()
            .expect("Failed to set port");
        info!("Port updated to {}", port);
    }
}

fn read_port() -> anyhow::Result<u32> {
    let config_path = std::env::var("GLUETUN_PATH")?;
    let port = fs::read_to_string(config_path)?;
    Ok(port.parse()?)
}

pub fn init_logger() {
    let env_name = "LOG_LEVEL";
    let env = Env::default()
        .filter_or(env_name, "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::builder().parse_env(env).init();
}

async fn connect_qbit() -> anyhow::Result<Qbit> {
    let user = std::env::var("QBIT_USER")?;
    let pwd = std::env::var("QBIT_PASSWORD")?;
    let host = std::env::var("QBIT_HOST")?;

    let credential = Credential::new(user, pwd);
    let api = Qbit::new(host.as_str(), credential);
    let vers = api.get_version().await?;
    info!(
        "Successfully connected to the qbittorrent instance with version: {}",
        vers
    );

    Ok(api)
}

async fn set_port(port: u32, api: &Qbit) -> anyhow::Result<()> {
    let mut prefs = api.get_preferences().await?;
    prefs.listen_port = Some(port as i64);
    api.set_preferences(prefs).await?;
    Ok(())
}

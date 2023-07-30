#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?
        .try_deserialize::<Settings>();
    
    settings
}

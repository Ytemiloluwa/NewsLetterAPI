use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {

    pub database : DatabaseSettings,
    pub application_port: u16
}
#[derive(serde::Deserialize)]

pub struct DatabaseSettings {

    pub username : String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    // initialise configuration reader

    let settings = config::Config::builder()
    // add configuration value from the configuration.yaml file

        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;
    // convert the configuration values it read into settings type

    settings.try_deserialize::<Settings>()

}

// Connecting to Postgres
impl DatabaseSettings {

    pub fn connection_string(&self) -> String{

        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name

        )
    }
    pub fn connection_string_without_db(&self) -> String { format!(
        "postgres://{}:{}@{}:{}",
        self.username, self.password, self.host, self.port
    )
    }
}
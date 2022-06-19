use config::Config;
use secrecy::{ExposeSecret, Secret};
#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

impl Settings {
    pub fn get_configuration() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("failed to get current directory");
        let configuration_directory = base_path.join("configuration");
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("APP_ENVIRONMENT");

        let settings = Config::builder()
            // base environment
            .add_source(config::File::from(configuration_directory.join("base")).required(true))
            // layer on specific environment
            .add_source(
                config::File::from(configuration_directory.join(environment.as_str()))
                    .required(true),
            )
            .add_source(config::Environment::with_prefix("app").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "Unknown environment: {}, use either 'local' or 'production'",
                other
            )),
        }
    }
}

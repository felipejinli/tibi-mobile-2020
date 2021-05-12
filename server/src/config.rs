use std::{path::PathBuf, time::Duration};

use hmac::Hmac;
use sha2::Sha256;

/// This config is available to all endpoints and should store common
/// data.
/// It is ensured that there will only ever be immutable access to this
/// once the server has started.
#[derive(Clone)]
pub struct Config {
    /// The private key for creating and authenticating `auth_token`s
    pub jwt_secret: Hmac<Sha256>,
    /// The duration for which a newly issue jwt token is valid for
    pub jwt_lifetime: Duration,
    /// The default timeout duration for when the SSO callback has not been
    /// called.
    pub sso_timeout: Duration,
    /// The extension to the SSO if there is a message to the callback
    pub sso_extension: Duration,
    /// The redirection url for the external SSO service without the GET
    /// params. It should be possible to append `?param_1=a&param_2=b`
    /// and for them to be valid GET params.
    pub sso_redirect_url: String,
    /// The client id for accessing the UCL api oauth system.
    pub oauth_client_id: String,
    /// The client secret for the UCL api oauth system.
    pub oauth_client_secret: String,
    /// The output directory for images uploaded and were they are served from.
    pub image_dir: PathBuf,
}

/// A config that is designed to be saved on a file (but not kept in git)
/// that contains private or config parameters that are very specific to each deployment.
/// Ideally you would load this config and then you could create a proper `Config` with
/// these values.
///
/// The datatypes for certain values are slightly different than in `Config` this is because
/// they all must be deserializable.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PrivateConfig {
    pub jwt_secret: String,
    pub sso_redirect_url: String,
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
}

impl PrivateConfig {
    /// Loads the config from file and returns Some(config) if it exists.
    /// If there is an error or it doesn't exist it will return None.
    /// If the config doesn't exist it will tell the user to create one.
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Option<PrivateConfig> {
        let path = path.as_ref();

        if !path.exists() {
            println!("Couldn't find config file at {:?}. You should create one now and fill in the template below:", path);
            println!(
                "{}",
                serde_json::to_string_pretty(&PrivateConfig {
                    jwt_secret: String::new(),
                    sso_redirect_url: String::new(),
                    oauth_client_id: String::new(),
                    oauth_client_secret: String::new()
                })
                .unwrap(),
            );
            return None;
        }

        let file = std::fs::File::open(path).expect("Couldn't open private config file");
        Some(serde_json::from_reader(file).expect("PrivateConfig file format was incorrect"))
    }
}

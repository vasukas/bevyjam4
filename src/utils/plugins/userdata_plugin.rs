use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

const USER_DIRECTORY: &str = "user"; // not for web; in current directory, i.e. project root
const COOKIE_EXPIRATION_DAYS: u64 = 7; // for web; how long before cookies are removed by the browser

const SECONDS_IN_DAY: u64 = 60 * 60 * 24;
const COOKIE_EXPIRATION: Duration = Duration::from_secs(SECONDS_IN_DAY * COOKIE_EXPIRATION_DAYS);

/// User-specific data, stored as cookies for web and as files for non-web.
///
/// Cookies have expiration date, so should be re-saved in each app run.
#[derive(Resource)]
pub struct Userdata {
    file_directory: String,

    #[allow(unused)]
    cookie_expiration: Duration,
}

impl Userdata {
    fn create() -> Self {
        let file_directory = USER_DIRECTORY.to_string();

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Err(err) = std::fs::create_dir_all(&file_directory) {
                error!("failed to create userdata directory: {err}");
            }
        }

        Self {
            file_directory,
            cookie_expiration: COOKIE_EXPIRATION,
        }
    }

    fn userdata_file(&self, name: &str) -> String {
        format!("{}/{name}.ron", self.file_directory)
    }

    /// Write userdata value. All errors are logged.
    ///
    /// Returns true on success.
    pub fn write<T: Serialize>(&self, name: &str, value: &T) -> bool {
        let value = match ron::ser::to_string_pretty(value, default()) {
            Ok(value) => value,
            Err(e) => {
                error!("failed to write userdata \"{name}\" - ron error: {e}");
                return false;
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = self.userdata_file(name);

            match std::fs::write(file, value) {
                Ok(_) => true,
                Err(e) => {
                    error!("failed to write userdata \"{name}\" - file error: {e}");
                    false
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let options =
                wasm_cookies::CookieOptions::default().expires_after(self.cookie_expiration);
            wasm_cookies::set(name, &value, &options);
            true
        }
    }

    /// Read userdata value. All errors are logged.
    ///
    /// Returns None on error.
    pub fn read<T: DeserializeOwned>(&self, name: &str) -> Option<T> {
        let value;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = self.userdata_file(name);

            value = match std::fs::read_to_string(file) {
                Ok(value) => value,
                Err(e) => {
                    warn!("failed to read userdata \"{name}\" - file error: {e}");
                    return None;
                }
            };
        }

        #[cfg(target_arch = "wasm32")]
        {
            value = match wasm_cookies::get(name) {
                Some(Ok(value)) => value,
                Some(Err(e)) => {
                    error!("failed to read userdata \"{name}\" - cookie decode error: {e}");
                    return None;
                }
                None => {
                    warn!("failed to read userdata \"{name}\" - no such cookie");
                    return None;
                }
            };
        }

        match ron::from_str(&value) {
            Ok(value) => Some(value),
            Err(e) => {
                error!("failed to read userdata \"{name}\" - ron error: {e}");
                None
            }
        }
    }

    /// Read userdata value or default, then write it (to update new/removed fields). All errors are logged.
    pub fn read_and_update<T: DeserializeOwned + Default + Serialize>(&self, name: &str) -> T {
        let value = self.read(name).unwrap_or_default();
        self.write(name, &value);
        value
    }
}

pub struct UserdataPlugin;

impl Plugin for UserdataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Userdata::create());
    }
}

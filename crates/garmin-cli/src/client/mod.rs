pub mod api;
pub mod oauth1;
pub mod sso;
pub mod tokens;

pub use api::{GarminClient, RequestLogger};
pub use oauth1::{OAuth1Signer, OAuthConsumer, OAuthToken};
pub use sso::SsoClient;
pub use tokens::{OAuth1Token, OAuth2Token};

use crate::error::{GarminError, Result};

/// International (global) Garmin domain.
pub const GLOBAL_DOMAIN: &str = "garmin.com";

/// Mainland China Garmin domain.
pub const CHINA_DOMAIN: &str = "garmin.cn";

/// Normalize a user-supplied region or domain into a Garmin domain.
///
/// Accepts `com`, `global`, `international`, `garmin.com`, `cn`, `china`,
/// `garmin.cn`. Anything else is rejected so a typo cannot silently pick a
/// different region.
pub fn resolve_domain(input: &str) -> Result<String> {
    let domain = match input.trim().to_ascii_lowercase().as_str() {
        "" | "com" | "global" | "international" | "garmin.com" => GLOBAL_DOMAIN,
        "cn" | "china" | "mainland" | "garmin.cn" => CHINA_DOMAIN,
        other => {
            return Err(GarminError::config(format!(
                "Unsupported Garmin domain '{}': use 'garmin.com' (international) or 'garmin.cn' (China)",
                other
            )))
        }
    };
    Ok(domain.to_string())
}

/// Current Garmin domain, resolved from the `GARMIN_DOMAIN` environment variable.
///
/// Falls back to [`GLOBAL_DOMAIN`] when unset or invalid.
pub fn current_domain() -> String {
    std::env::var("GARMIN_DOMAIN")
        .ok()
        .filter(|d| !d.trim().is_empty())
        .and_then(|d| resolve_domain(&d).ok())
        .unwrap_or_else(|| GLOBAL_DOMAIN.to_string())
}

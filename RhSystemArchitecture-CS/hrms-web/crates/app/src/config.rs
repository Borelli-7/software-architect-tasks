//! Compile-time client configuration.

/// Base URL of the API gateway the SPA talks to.
/// Overridable at build time via the `HRMS_API_BASE_URL` env var.
pub fn api_base_url() -> String {
    option_env!("HRMS_API_BASE_URL")
        .unwrap_or("http://localhost:8080/api")
        .to_string()
}

/// OIDC issuer (Keycloak realm) URL.
pub fn oidc_issuer_url() -> String {
    option_env!("HRMS_OIDC_ISSUER")
        .unwrap_or("http://localhost:8081/realms/hrms")
        .to_string()
}

/// OIDC public client id for this SPA.
pub fn oidc_client_id() -> String {
    option_env!("HRMS_OIDC_CLIENT_ID")
        .unwrap_or("hrms-web")
        .to_string()
}

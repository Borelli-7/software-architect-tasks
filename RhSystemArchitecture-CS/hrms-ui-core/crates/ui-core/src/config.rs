//! Compile-time client configuration for the native apps.

/// Base URL of the API gateway.
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

/// OIDC public client id for the native apps.
pub fn oidc_client_id() -> String {
    option_env!("HRMS_OIDC_CLIENT_ID")
        .unwrap_or("hrms-native")
        .to_string()
}

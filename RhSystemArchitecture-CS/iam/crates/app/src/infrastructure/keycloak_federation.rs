//! Keycloak adapter federating identities to the corporate IdP.

use async_trait::async_trait;

use crate::application::ports::DirectoryFederation;
use crate::domain::user::User;

pub struct KeycloakFederation {
    base_url: String,
    realm: String,
}

impl KeycloakFederation {
    pub fn new(base_url: String, realm: String) -> Self {
        Self { base_url, realm }
    }
}

#[async_trait]
impl DirectoryFederation for KeycloakFederation {
    async fn provision(&self, _user: &User) -> anyhow::Result<()> {
        todo!("create the Keycloak user in the configured realm")
    }

    async fn deprovision(&self, _user: &User) -> anyhow::Result<()> {
        todo!("disable the Keycloak user in the configured realm")
    }
}

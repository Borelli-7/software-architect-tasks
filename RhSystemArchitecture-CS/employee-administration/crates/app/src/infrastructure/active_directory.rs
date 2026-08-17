//! Active Directory / LDAP adapter implementing the identity provisioning port.

use async_trait::async_trait;

use crate::application::ports::IdentityProvisioner;
use crate::domain::employee::Employee;

pub struct ActiveDirectoryClient {
    base_url: String,
}

impl ActiveDirectoryClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }
}

#[async_trait]
impl IdentityProvisioner for ActiveDirectoryClient {
    async fn provision(&self, _employee: &Employee) -> anyhow::Result<()> {
        let _ = &self.base_url;
        todo!("call AD/LDAP to create the user account")
    }

    async fn deprovision(&self, _employee: &Employee) -> anyhow::Result<()> {
        todo!("call AD/LDAP to disable the user account")
    }
}

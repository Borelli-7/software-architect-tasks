//! Message templates rendered before dispatch.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::NotificationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemplateId(pub Uuid);

/// A named, parameterised message body (e.g. Handlebars-style placeholders).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub subject: Option<String>,
    pub body: String,
}

impl Template {
    /// Renders the template body against the supplied key/value parameters.
    pub fn render(&self, _params: &[(String, String)]) -> Result<String, NotificationError> {
        todo!("substitute placeholders and return the rendered body")
    }
}

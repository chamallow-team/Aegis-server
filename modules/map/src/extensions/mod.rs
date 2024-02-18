use serde::{Deserialize, Serialize};

pub type ExtensionID = String;

/// TODO
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Extension {
    id: ExtensionID
}
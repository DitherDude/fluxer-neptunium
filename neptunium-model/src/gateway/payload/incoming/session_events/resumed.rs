use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
pub struct Resumed {
    pub country_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    /// Possible extra data due to this data being undocumented.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

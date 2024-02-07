use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize)]
struct PackageInfo {
    file_name: Option<Value>,
    version: Option<Value>,
    description: Option<Value>,
    url: Option<Value>,
    hash: Option<Value>,
}

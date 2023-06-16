use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub pubdir: Option<String>,
    pub package: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub author: Option<String>,
    pub config: Option<PackageConfig>,
    pub packages: Option<Vec<String>>,
    pub package_map: Option<HashMap<String, String>>,
    pub modules: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Var {
    Constant,
    Global,
    Stated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    None,
    Val(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: VariableValue,
    pub var_type: Var,
}

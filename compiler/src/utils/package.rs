use serde::{Deserialize, Serialize};
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
pub struct Variable {
    pub name: String,
    pub value: Option<String>,
    pub var_type: Var,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Log {
    Variables(Vec<Variable>),
    Code(Vec<String>),
}

pub type InterpreterLog = HashMap<String, Log>;

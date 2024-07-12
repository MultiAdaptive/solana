use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum NodeName {
    ExecuteNode,
}

impl NodeName {
    fn as_str(&self) -> &str {
        match self {
            NodeName::ExecuteNode => "execute",
            _ => "",
        }
    }
}

impl std::fmt::Display for NodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeName::ExecuteNode => write!(f, "execute"),
        }
    }
}

impl From<NodeName> for String {
    fn from(s: NodeName) -> Self {
        match s {
            NodeName::ExecuteNode => "execute".to_string(),
        }
    }
}


impl From<String> for NodeName {
    fn from(s: String) -> Self {
        match s {
            s if s == "execute" => NodeName::ExecuteNode,
            _ => unreachable!(),
        }
    }
}


impl FromStr for NodeName {
    type Err = ();

    fn from_str(s: &str) -> Result<NodeName, Self::Err> {
        match s {
            "execute" => Ok(NodeName::ExecuteNode),
            _ => Err(()),
        }
    }
}



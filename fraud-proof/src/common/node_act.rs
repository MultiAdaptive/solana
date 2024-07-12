use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum NodeAct {
    CreateBrief,
    SubmitBrief,
    GenerateData,
}

impl NodeAct {
    fn as_str(&self) -> &str {
        match self {
            NodeAct::CreateBrief => "create-brief",
            NodeAct::SubmitBrief => "submit-brief",
            NodeAct::GenerateData => "generate-data",
            _ => "",
        }
    }
}

impl std::fmt::Display for NodeAct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeAct::CreateBrief => write!(f, "create-brief"),
            NodeAct::SubmitBrief => write!(f, "submit-brief"),
            NodeAct::GenerateData => write!(f, "generate-data"),
        }
    }
}

impl From<NodeAct> for String {
    fn from(s: NodeAct) -> Self {
        match s {
            NodeAct::CreateBrief => "create-brief".to_string(),
            NodeAct::SubmitBrief => "submit-brief".to_string(),
            NodeAct::GenerateData => "generate-data".to_string(),
        }
    }
}


impl From<String> for NodeAct {
    fn from(s: String) -> Self {
        match s {
            s if s == "create-brief" => NodeAct::CreateBrief,
            s if s == "submit-brief" => NodeAct::SubmitBrief,
            s if s == "generate-data" => NodeAct::GenerateData,
            _ => unreachable!(),
        }
    }
}


impl FromStr for NodeAct {
    type Err = ();

    fn from_str(s: &str) -> Result<NodeAct, Self::Err> {
        match s {
            "create-brief" => Ok(NodeAct::CreateBrief),
            "submit-brief" => Ok(NodeAct::SubmitBrief),
            "generate-data" => Ok(NodeAct::GenerateData),
            _ => Err(()),
        }
    }
}

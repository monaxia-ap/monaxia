use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct WebfingerQuery {
    pub resource: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WellknownWebfinger {
    pub subject: String,
    pub links: Vec<WellknownWebfingerLink>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WellknownWebfingerLink {
    pub rel: String,
    pub r#type: String,
    pub href: Url,
}

#[derive(Debug, Clone, Serialize)]
pub struct WellknownNodeinfo {
    pub links: Vec<WellknownNodeinfoLink>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WellknownNodeinfoLink {
    pub rel: String,
    pub href: Url,
}

#[derive(Debug, Clone, Serialize)]
pub struct Nodeinfo {
    pub version: String,
    pub software: NodeinfoSoftware,
    pub protocols: Vec<String>,
    pub services: NodeinfoServices,
    pub open_registrations: bool,
    pub usage: NodeinfoUsage,
    pub metadata: NodeinfoMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeinfoSoftware {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeinfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeinfoUsage {
    pub users: NodeinfoUsageUsers,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeinfoUsageUsers {
    pub total: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeinfoMetadata {}

use chrono::{DateTime, Utc};
use http_types::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use oro_manifest::{OroManifest, PersonField};
use oro_node_semver::Version;

/// A serializable representation of a Packument -- the toplevel metadata
/// object containing information about package versions, dist-tags, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Packument {
    pub versions: HashMap<Version, VersionMetadata>,
    #[serde(default)]
    pub time: HashMap<String, DateTime<Utc>>,
    #[serde(default, rename = "dist-tags")]
    pub tags: HashMap<String, Version>,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

/// A manifest for an individual package version.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionMetadata {
    #[serde(default)]
    pub maintainers: Vec<PersonField>,
    #[serde(rename = "_npmUser")]
    pub npm_user: Option<Human>,
    pub dist: Dist,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: Option<bool>,
    pub deprecated: Option<String>,

    #[serde(flatten)]
    pub manifest: OroManifest,
}

/// Representation for the `bin` field in package manifests.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Hash(HashMap<String, String>),
}

/// Represents a human!
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Human {
    pub name: String,
    pub email: Option<String>,
}

/// Distribution information for a particular package version.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dist {
    pub shasum: String,
    pub tarball: Url,

    pub integrity: Option<String>,
    #[serde(rename = "fileCount")]
    pub file_count: Option<usize>,
    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<usize>,
    #[serde(rename = "npm-signature")]
    pub npm_signature: Option<String>,

    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

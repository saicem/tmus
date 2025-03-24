use crate::config::radix::RadixTree;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    exclude: Vec<ExcludeRuleItem>,
    /// Compare to exclude, has higher priority
    include: Vec<IncludeRuleItem>,
    merge: Vec<MergeRuleItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExcludeRuleItem {
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncludeRuleItem {
    path: String,
}

/// Convert path prefix with "path" to "to_path"
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MergeRuleItem {
    path: String,
    to_path: String,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            exclude: vec![
                ExcludeRuleItem {
                    path: String::from("C:\\Windows\\SystemApps"),
                },
                ExcludeRuleItem {
                    path: String::from("C:\\Windows\\System32"),
                },
                ExcludeRuleItem {
                    path: String::from("~\\AppData\\Local\\Temp"),
                },
            ],
            include: Default::default(),
            merge: Default::default(),
        }
    }
}

static EXCLUDE: OnceLock<RadixTree<()>> = OnceLock::new();
static INCLUDE: OnceLock<RadixTree<()>> = OnceLock::new();
static MERGE: OnceLock<RadixTree<String>> = OnceLock::new();

pub fn init_rule(rule: &Rule) {
    let mut exclude = RadixTree::new();
    let mut include = RadixTree::new();
    let mut merge = RadixTree::new();
    for item in &rule.exclude {
        exclude.insert(&expand_path(&item.path), Some(()));
    }
    for item in &rule.include {
        include.insert(&expand_path(&item.path), Some(()));
    }
    for item in &rule.merge {
        merge.insert(&expand_path(&item.path), Some(expand_path(&item.to_path)));
    }
    EXCLUDE.set(exclude).unwrap();
    INCLUDE.set(include).unwrap();
    MERGE.set(merge).unwrap();
}

/// Expand ~ to user home path
fn expand_path(path: &str) -> String {
    if path.starts_with("~") {
        static HOME_DIR: OnceLock<String> = OnceLock::new();
        let home_dir = HOME_DIR.get_or_init(|| {
            dirs_next::home_dir()
                .expect("Can't get user home.")
                .to_str()
                .expect("Can't convert home path to str.")
                .to_owned()
        });
        path.replace("~", home_dir)
    } else {
        path.to_owned()
    }
}

pub fn is_exclude(app_path: &str) -> bool {
    EXCLUDE
        .get()
        .unwrap()
        .longest_prefix_meta(app_path)
        .is_some()
}

pub fn is_include(app_path: &str) -> bool {
    INCLUDE
        .get()
        .unwrap()
        .longest_prefix_meta(app_path)
        .is_some()
}

pub fn get_merged_path(app_path: &str) -> Option<String> {
    MERGE.get().unwrap().longest_prefix_meta(app_path)
}

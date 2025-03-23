use crate::config::radix::RadixTree;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    exclude: Vec<String>,
    /// Compare to exclude, has higher priority
    include: Vec<String>,
    merge: HashMap<String, HashSet<String>>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            exclude: vec![
                String::from("C:\\Windows\\SystemApps"),
                String::from("C:\\Windows\\System32"),
                String::from("~\\AppData\\Local\\Temp"),
            ],
            include: Default::default(),
            merge: Default::default(),
        }
    }
}

static EXCLUDE: OnceLock<RadixTree<()>> = OnceLock::new();
static INCLUDE: OnceLock<RadixTree<()>> = OnceLock::new();
static MERGE: OnceLock<RadixTree<Arc<String>>> = OnceLock::new();

pub fn init_rule(rule: &Rule) {
    let mut exclude = RadixTree::new();
    let mut include = RadixTree::new();
    let mut merge = RadixTree::new();
    for path in &rule.exclude {
        exclude.insert(&expand_path(path), Some(()));
    }
    for path in &rule.include {
        include.insert(&expand_path(path), Some(()));
    }
    for (merged_path, need_merge_paths) in &rule.merge {
        let merged_path = Arc::new(expand_path(merged_path));
        for path in need_merge_paths {
            merge.insert(&expand_path(path), Some(Arc::clone(&merged_path)));
        }
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

pub fn get_merged_path(app_path: &str) -> Option<Arc<String>> {
    MERGE.get().unwrap().longest_prefix_meta(app_path)
}

mod test {
    use super::*;

    #[test]
    fn test_exclude() {
        let app_path =
            "C:\\Windows\\SystemApps\\MicrosoftWindows.Client.CBS_cw5n1h2txyewy\\SearchHost.exe";
        let mut exclude = RadixTree::new();
        for path in Rule::default().exclude {
            exclude.insert(&expand_path(&path), Some(()));
        }
        exclude.longest_prefix_meta(app_path);
    }
}

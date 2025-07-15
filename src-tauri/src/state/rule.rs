use crate::app::constant::rule_file_path;
use crate::state::radix::RadixTree;
use crate::util::load_json;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

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

#[derive(Debug)]
pub struct RuleRadixTree {
    exclude: RadixTree<()>,
    include: RadixTree<()>,
    merge: RadixTree<String>,
}

impl RuleRadixTree {
    pub fn new(rule: &Rule) -> Self {
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
        Self {
            exclude,
            include,
            merge,
        }
    }

    pub fn is_exclude(&self, app_path: &str) -> bool {
        self.exclude.longest_prefix_meta(app_path).is_some()
    }

    pub fn is_include(&self, app_path: &str) -> bool {
        self.include.longest_prefix_meta(app_path).is_some()
    }

    pub fn get_merged_path(&self, app_path: &str) -> Option<String> {
        self.merge.longest_prefix_meta(app_path)
    }

    pub fn filter(&self, app_path: &str) -> Option<String> {
        if app_path.is_empty() || (self.is_exclude(&app_path) && !self.is_include(&app_path)) {
            return None;
        }
        self.get_merged_path(&app_path)
            .or(Some(app_path.to_owned()))
    }
}

static RULE: OnceLock<Mutex<Rule>> = OnceLock::new();

pub fn get_rule() -> &'static Mutex<Rule> {
    RULE.get_or_init(|| Mutex::new(load_json(rule_file_path())))
}

static RULE_RADIX_TREE: OnceLock<Mutex<RuleRadixTree>> = OnceLock::new();

pub fn get_rule_radix_tree() -> &'static Mutex<RuleRadixTree> {
    RULE_RADIX_TREE.get_or_init(|| Mutex::new(RuleRadixTree::new(&get_rule().lock().unwrap())))
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

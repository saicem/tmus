use std::collections::HashMap;

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|(ca, cb)| ca == cb)
        .count()
}

fn char_count_to_byte_index(s: &str, char_count: usize) -> usize {
    s.char_indices()
        .nth(char_count)
        .map_or(s.len(), |(idx, _)| idx)
}

#[derive(Debug)]
struct RadixNode<T> {
    prefix: String,
    children: HashMap<char, Box<RadixNode<T>>>,
    meta: Option<T>,
}

impl<T: Clone> RadixNode<T> {
    fn new(prefix: &str) -> Self {
        RadixNode {
            prefix: prefix.to_string(),
            children: HashMap::new(),
            meta: None,
        }
    }

    fn insert(&mut self, s: &str, meta: Option<T>) {
        let common_chars = common_prefix(&self.prefix, s);

        debug_assert!(
            common_chars != 0 || self.prefix.is_empty(),
            "Unexpected case: non-root node with zero common prefix"
        );

        let split_at = char_count_to_byte_index(&self.prefix, common_chars);
        if common_chars < self.prefix.chars().count() {
            let (common_part, remaining) = self.prefix.split_at(split_at);

            let new_child = RadixNode {
                prefix: remaining.to_string(),
                children: std::mem::take(&mut self.children),
                meta: std::mem::take(&mut self.meta),
            };

            self.prefix = common_part.to_string();
            self.meta = None;

            let first_char = new_child.prefix.chars().next().unwrap();
            self.children.insert(first_char, Box::new(new_child));
        }

        if split_at == s.len() {
            self.meta = meta;
            return;
        }

        let remaining_s = &s[split_at..];

        let first_char = remaining_s.chars().next().unwrap();
        match self.children.get_mut(&first_char) {
            Some(child) => child.insert(remaining_s, meta),
            None => {
                let mut new_node = RadixNode::new(remaining_s);
                new_node.meta = meta;
                self.children.insert(first_char, Box::new(new_node));
            }
        }
    }

    #[allow(dead_code)]
    fn contain(&self, s: &str) -> bool {
        let common_chars = common_prefix(&self.prefix, s);
        if common_chars < self.prefix.chars().count() {
            return false;
        }

        let split_at = char_count_to_byte_index(&self.prefix, common_chars);
        let remaining_s = &s[split_at..];
        if remaining_s.is_empty() {
            return self.meta.is_some();
        }

        match remaining_s.chars().next() {
            Some(c) => self
                .children
                .get(&c)
                .map_or(false, |child| child.contain(remaining_s)),
            None => false,
        }
    }

    fn longest_prefix_meta(&self, s: &str) -> Option<T> {
        let common_chars = common_prefix(&self.prefix, s);
        if common_chars < self.prefix.chars().count() {
            return None;
        }

        let split_at = char_count_to_byte_index(&self.prefix, common_chars);
        let remaining_s = &s[split_at..];
        if remaining_s.is_empty() {
            return self.meta.clone();
        }

        remaining_s
            .chars()
            .next()
            .and_then(|c| {
                self.children
                    .get(&c)
                    .and_then(|child| child.longest_prefix_meta(remaining_s))
            })
            .or(self.meta.clone())
    }
}

#[derive(Debug)]
pub struct RadixTree<T> {
    root: RadixNode<T>,
}

impl<T: Clone> RadixTree<T> {
    pub fn new() -> Self {
        RadixTree {
            root: RadixNode::new(""),
        }
    }

    pub fn insert(&mut self, s: &str, meta: Option<T>) {
        if s.is_empty() {
            self.root.meta = meta;
            return;
        }
        self.root.insert(s, meta);
    }

    #[allow(dead_code)]
    pub fn contain(&self, s: &str) -> bool {
        if s.is_empty() {
            return self.root.meta.is_some();
        }
        self.root.contain(s)
    }

    pub fn longest_prefix_meta(&self, s: &str) -> Option<T> {
        if s.is_empty() {
            return self.root.meta.clone();
        }
        self.root.longest_prefix_meta(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_tree_basic_insert_and_lookup() {
        let mut tree = RadixTree::new();

        tree.insert("apple", Some(()));
        tree.insert("apply", Some(()));
        tree.insert("app", Some(()));
        tree.insert("中文", Some(()));

        assert!(tree.contain("apple"));
        assert!(tree.contain("apply"));
        assert!(tree.contain("app"));
        assert!(tree.contain("中文"));

        assert!(!tree.contain("ap"));
        assert!(!tree.contain("applications"));
        assert!(!tree.contain("中"));
    }

    #[test]
    fn test_radix_tree_edge_cases() {
        let mut tree = RadixTree::new();

        tree.insert("", Some(()));
        assert!(tree.contain(""));

        tree.insert("a", Some(()));
        assert!(tree.contain("a"));

        tree.insert("app", Some(()));
        tree.insert("apple", Some(()));
        assert!(tree.contain("app"));
        assert!(tree.contain("apple"));
    }

    #[test]
    fn test_radix_tree_longest_prefix_matching() {
        let mut tree = RadixTree::new();
        tree.insert("app", Some(1));
        tree.insert("apple", Some(2));
        tree.insert("applet", Some(3));
        assert_eq!(tree.longest_prefix_meta("app"), Some(1));
        assert_eq!(tree.longest_prefix_meta("appl"), Some(1));
        assert_eq!(tree.longest_prefix_meta("apple"), Some(2));
        assert_eq!(tree.longest_prefix_meta("applet"), Some(3));
    }
}

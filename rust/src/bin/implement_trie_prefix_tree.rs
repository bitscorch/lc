//! 208. Implement Trie (Prefix Tree)
//! Medium | Hash Table | String | Design | Trie
//! https://leetcode.com/problems/implement-trie-prefix-tree/
//!
//! A [**trie**](https://en.wikipedia.org/wiki/Trie) (pronounced as "try") or
//! **prefix tree** is a tree data structure used to efficiently store and
//! retrieve keys in a dataset of strings. There are various applications of
//! this data structure, such as autocomplete and spellchecker.
//!
//! Implement the Trie class:
//!
//! * `Trie()` Initializes the trie object.
//! * `void insert(String word)` Inserts the string `word` into the trie.
//! * `boolean search(String word)` Returns `true` if the string `word` is in the trie (i.e., was inserted before), and `false` otherwise.
//! * `boolean startsWith(String prefix)` Returns `true` if there is a previously inserted string `word` that has the prefix `prefix`, and `false` otherwise.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
//! [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
//! Output
//! [null, null, true, false, true, null, true]
//!
//! Explanation
//! Trie trie = new Trie();
//! trie.insert("apple");
//! trie.search("apple");   // return True
//! trie.search("app");     // return False
//! trie.startsWith("app"); // return True
//! trie.insert("app");
//! trie.search("app");     // return True
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= word.length, prefix.length <= 2000`
//! * `word` and `prefix` consist only of lowercase English letters.
//! * At most `3 * 10<sup>4</sup>` calls **in total** will be made to `insert`, `search`, and `startsWith`.

struct Solution;

struct Trie {
    // children: HashMap<char, Trie>,
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for &b in word.as_bytes() {
            let i = (b - b'a') as usize;
            node = node.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for &b in word.as_bytes() {
            let i = (b - b'a') as usize;
            match node.children[i].as_ref() {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for &b in prefix.as_bytes() {
            let i = (b - b'a') as usize;
            match node.children[i].as_ref() {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string())); // inserted word
        assert!(!trie.search("app".to_string())); // a prefix, never inserted as a word
        assert!(trie.starts_with("app".to_string())); // ...but it IS a prefix
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string())); // now it's a word too
    }

    #[test]
    fn edges() {
        let mut trie = Trie::new();

        // empty trie: nothing matches
        assert!(!trie.search("a".to_string()));
        assert!(!trie.starts_with("a".to_string()));

        trie.insert("hello".to_string());

        // the key trie distinction: a prefix is NOT a word
        assert!(trie.starts_with("hell".to_string()));
        assert!(!trie.search("hell".to_string()));

        // a full word is also a prefix of itself
        assert!(trie.starts_with("hello".to_string()));
        assert!(trie.search("hello".to_string()));

        // shares a prefix but was never inserted
        assert!(!trie.search("help".to_string()));
        assert!(!trie.starts_with("world".to_string()));

        // overlapping words coexist
        trie.insert("help".to_string());
        assert!(trie.search("help".to_string()));
        assert!(trie.search("hello".to_string())); // still there
        assert!(trie.starts_with("hel".to_string())); // shared prefix

        // single-character word
        trie.insert("a".to_string());
        assert!(trie.search("a".to_string()));
        assert!(trie.starts_with("a".to_string()));
    }
}

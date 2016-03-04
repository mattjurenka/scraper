//! Node references.

use std::ops::Deref;

use ego_tree;
use ego_tree::iter::{Traverse, Edge};

use {Node, Selector};

/// Wrapper around a reference to an HTML node.
///
/// This wrapper implements the `Element` trait from the `selectors` crate, which allows it to be
/// matched against CSS selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef<'a>(pub ego_tree::NodeRef<'a, Node>);

impl<'a> Deref for NodeRef<'a> {
    type Target = ego_tree::NodeRef<'a, Node>;

    fn deref(&self) -> &ego_tree::NodeRef<'a, Node> {
        &self.0
    }
}

impl<'a> NodeRef<'a> {
    /// Returns an iterator over child elements matching a selector.
    pub fn select<'b>(&self, selector: &'b Selector) -> Select<'a, 'b> {
        let mut inner = self.traverse();
        inner.next(); // Skip Open(self).

        Select {
            inner: inner,
            selector: selector,
        }
    }

    /// Returns an iterator over descendent text nodes.
    pub fn text(&self) -> Text {
        Text {
            inner: self.traverse(),
        }
    }
}

/// Iterator over child elements matching a selector.
#[derive(Debug, Clone)]
pub struct Select<'a, 'b> {
    inner: Traverse<'a, Node>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = NodeRef<'a>;

    fn next(&mut self) -> Option<NodeRef<'a>> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                let node_ref = NodeRef(node);
                if node.value().is_element() && self.selector.matches(&node_ref) {
                    return Some(node_ref);
                }
            }
        }
        None
    }
}

/// Iterator over descendent text nodes.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    inner: Traverse<'a, Node>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let &Node::Text(ref text) = node.value() {
                    return Some(&*text);
                }
            }
        }
        None
    }
}

mod element;

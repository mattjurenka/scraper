//! DOM implementation.

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt;
use std::ops::Deref;

use html5ever::Attribute;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
use string_cache::QualName;
use tendril::StrTendril;
use typed_arena::Arena;

/// Arena-allocated DOM.
pub struct Dom<'a> {
    arena: Arena<TreeNode<'a>>,

    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The document root node.
    pub document: &'a TreeNode<'a>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,
}

/// A node in the DOM tree.
#[derive(Debug)]
pub struct TreeNode<'a> {
    /// The DOM node.
    pub node: (),

    /// The parent node.
    pub parent: Cell<Option<&'a TreeNode<'a>>>,

    /// The first and last children.
    pub children: Cell<Option<(&'a TreeNode<'a>, &'a TreeNode<'a>)>>,

    /// The next sibling.
    pub next_sibling: Cell<Option<&'a TreeNode<'a>>>,

    /// The previous sibling.
    pub prev_sibling: Cell<Option<&'a TreeNode<'a>>>,
}

/// A reference to a `TreeNode`.
#[derive(Debug, Clone)]
pub struct Handle<'a>(&'a TreeNode<'a>);
impl<'a> Deref for Handle<'a> {
    type Target = TreeNode<'a>;
    fn deref(&self) -> &TreeNode<'a> { self.0 }
}

impl<'a> fmt::Debug for Dom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Dom")
            .field("errors", &self.errors)
            .field("document", &self.document)
            .field("quirks_mode", &self.quirks_mode)
            .finish()
    }
}

#[allow(trivial_casts)]
fn same_node<'a>(a: &TreeNode<'a>, b: &TreeNode<'a>) -> bool {
    a as *const _ == b as *const _
}

#[allow(unused_variables)]
impl<'a> TreeSink for Dom<'a> {
    type Handle = Handle<'a>;

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    fn get_document(&mut self) -> Handle<'a> {
        Handle(self.document)
    }

    fn get_template_contents(&self, target: Self::Handle) -> Self::Handle {
        unimplemented!()
    }

    fn same_node(&self, x: Handle<'a>, y: Handle<'a>) -> bool {
        same_node(x.0, y.0)
    }

    fn elem_name(&self, target: Self::Handle) -> QualName {
        unimplemented!()
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Self::Handle {
        unimplemented!()
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        unimplemented!()
    }

    fn append(&mut self, parent: Self::Handle, child: NodeOrText<Self::Handle>) {
        unimplemented!();
    }

    fn append_before_sibling(&mut self, sibling: Self::Handle, new_node: NodeOrText<Self::Handle>) -> Result<(), NodeOrText<Self::Handle>> {
        unimplemented!()
    }

    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril, system_id: StrTendril) {
        unimplemented!();
    }

    fn add_attrs_if_missing(&mut self, target: Self::Handle, attrs: Vec<Attribute>) {
        unimplemented!();
    }

    #[allow(trivial_casts)]
    fn remove_from_parent(&mut self, target: Handle<'a>) {
        let node = target.deref();
        let parent = match node.parent.get() {
            Some(p) => p,
            None => return,
        };
        let prev_sibling = node.prev_sibling.get();
        let next_sibling = node.next_sibling.get();

        // Update sibling refs.
        if let Some(sibling) = prev_sibling {
            sibling.next_sibling.set(node.next_sibling.get());
        }
        if let Some(sibling) = next_sibling {
            sibling.prev_sibling.set(node.prev_sibling.get());
        }

        // Update parent refs.
        if prev_sibling.is_none() && next_sibling.is_none() {
            parent.children.set(None);
        } else {
            let mut parent_children = parent.children.get().unwrap();
            if same_node(parent_children.0, node) {
                parent_children.0 = next_sibling.unwrap();
            }
            if same_node(parent_children.1, node) {
                parent_children.1 = prev_sibling.unwrap();
            }
            parent.children.set(Some(parent_children));
        }

        // Orphan node.
        node.parent.set(None);
        node.next_sibling.set(None);
        node.prev_sibling.set(None);
    }

    fn reparent_children(&mut self, node: Self::Handle, new_parent: Self::Handle) {
        unimplemented!();
    }

    fn mark_script_already_started(&mut self, node: Self::Handle) {
        unimplemented!();
    }
}
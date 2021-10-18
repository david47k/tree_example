// Tree example in Rust by David Atkinson
// Can be used multithreaded

use std::sync::{Arc,RwLock,Weak};

struct TreeNode<T> where T: Clone {
	pub parent: Option<TreeNodeWeak<T>>,
	pub children: Vec<TreeNodeRef<T>>,
	pub val: T,
}

#[derive(Clone)]
pub struct TreeNodeRef<T> where T: Clone {
	inner: Arc<RwLock<TreeNode<T>>>,
}

impl<T> TreeNodeRef<T> where T: Clone + PartialEq {
	pub fn val(&self) -> T {					// get the val of this node
		self.inner.read().unwrap().val.clone()
	}
	pub fn set_val(&mut self, val: T) {			// set the val of this node
		self.inner.write().unwrap().val = val;
	}
	pub fn parent(&self) -> Option<Self> { 		// return strong form of parent
		let inner = self.inner.read().unwrap();
		let p = inner.parent.clone();
		match p {
			None => None,
			Some(r) => Some(r.upgrade()),
		}
	}
	pub fn len(&self) -> usize {				// return len of the children vec
		self.inner.read().unwrap().children.len()
	}
	pub fn depth(&self) -> usize {				// count back to the root to get depth
		let mut count = 0;
		let mut nptr = self.clone();
		while !nptr.is_root() {
			count += 1;
			nptr = nptr.parent().unwrap();
		}
		count
	}
	pub fn is_root(&self) -> bool {
		self.inner.read().unwrap().parent.is_none()
	}
	pub fn has_children(&self) -> bool {
		self.inner.read().unwrap().children.len() > 0
	}
	pub fn push(&mut self, v: T) -> Self {		// push a child into the children vec, return a pointer to the child TreeNodeRef
		let mut inner = self.inner.write().unwrap();
		let n = Self {
			inner: Arc::new(RwLock::new(TreeNode {
				parent: Some(self.clone().downgrade()),
				children: Vec::<TreeNodeRef::<T>>::new(),
				val: v,
			})),
		};
		inner.children.push(n.clone());
		n
	}
	pub fn push_ref(&mut self, r: TreeNodeRef<T>) -> Self {		// push a child as a ref... returns the same ref
		let mut inner = self.inner.write().unwrap();
		inner.children.push(r.clone());
		r
	}
	pub fn move_to(&mut self, dest: &mut TreeNodeRef::<T>) {	// move a node from its current parent to a different parent
		// Lock ourselves
		let mut inner = self.inner.write().unwrap();

		// Add ourselves to the new parent
		let mut dest_inner = dest.inner.write().unwrap();
		dest_inner.children.push(self.clone());
		// We don't release the lock yet, because we don't want to show up under two different parents!

		// Remove ourselves from our current parent
		let parent = inner.parent.clone();
		match parent {
			Some(p) => {
				let strong = p.inner.upgrade().unwrap();
				let mut p_inner = strong.write().unwrap();
				// Remove all references to ourself from the parent's children vec
				p_inner.children.retain(|n| !Arc::ptr_eq(&n.inner, &self.inner));
			},
			None => {},
		};

		// Update our our own parent reference
		inner.parent = Some(dest.clone().downgrade());
	}
	pub fn new(v: T) -> Self {								// create a new root node
		Self {
			inner: Arc::new(RwLock::new(TreeNode {
				parent: None,
				children: Vec::<TreeNodeRef::<T>>::new(),
				val: v,
			})),
		}
	}
	pub fn push_vertical(&mut self, v: &Vec::<T>) -> TreeNodeRef<T> { // this pushes child, then their child, then their child etc.
		let mut n = self.clone();
		for i in 0..v.len() {
			n = n.push(v[i].clone());
		}
		n
	}
	pub fn push_children(&mut self, v: &Vec::<T>) -> TreeNodeRef<T> {
		if v.len() < 1 { panic!("Attempting to push empty vec"); }
		let mut n = self.clone();	// placeholder, last returned ref will be returned, not this
		for i in 0..v.len() {
			n = self.push(v[i].clone());
		}
		n
	}
	pub fn children(&self) -> Vec::<TreeNodeRef::<T>> { 	// return a clone of the children vec
		self.inner.read().unwrap().children.clone()
	}
	pub fn to_vertical_vec(&self) -> Vec<T> {				// find out our parent, and their parent, and their parent etc.
		let mut v = Vec::<T>::new();
		let mut n = self.clone();
		while !n.is_root() {
			v.push(n.val());
			n = n.parent().unwrap();
		}
		v
	}
	pub fn print_tree_size(&self) {							// print some info on the tree, useful for debugging
		// first find root
		let mut nptr = self.clone();
		while nptr.len() > 0 {
			nptr = nptr.parent().unwrap();
		}
		// then go down all the paths
		let mut max_strong = 0;
		let mut count = 1;
		let mut childs_outer: Vec<TreeNodeRef<T>> = vec![ nptr ];
		let mut childs_inner: Vec<TreeNodeRef<T>> = vec![];
		while childs_outer.len() > 0 {
			let pop = childs_outer.pop().unwrap();
			for c in pop.inner.read().unwrap().children.iter() {
					count += 1;
					let sc = Arc::strong_count(&c.inner);
					if sc > max_strong {
						max_strong = sc;
					}
					childs_inner.push(c.clone());
			}
			if childs_outer.len() == 0 {
				childs_outer.append(&mut childs_inner);
			}
		}
		println!("Tree has {} nodes, {} MB, max strong references {}", count, count * std::mem::size_of::<TreeNode::<T>>() / (1024*1024), max_strong);
	}
	pub fn downgrade(self) -> TreeNodeWeak::<T> {		// downgrade this ref to a weak form (weak form doesn't affect reference count)
		TreeNodeWeak::<T> {
			inner: Arc::downgrade(&self.inner),
		}
	}
	pub fn find(&self, val: T) -> Option<TreeNodeRef<T>> {	// find the child node with the given value, and return a ref to it
		let inner = self.inner.read().unwrap();
		let r = inner.children.iter().find(|&n| n.val()==val);
		match r {
			Some(n) => Some(n.clone()),
			None => None,
		}
	}
} 

#[derive(Clone)]
pub struct TreeNodeWeak<T> where T: Clone {
	inner: Weak<RwLock<TreeNode<T>>>,
}

impl<T> TreeNodeWeak<T> where T: Clone {
	pub fn upgrade(self) -> TreeNodeRef<T> {	// upgrade this weak ref into a strong one
		TreeNodeRef {
			inner: self.inner.upgrade().unwrap()
		}
	}
}

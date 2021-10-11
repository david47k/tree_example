# Tree data structure example in rust

This source code demonstrates a tree structure in rust. The tree is mutable, there is no 'unsafe' code, and the tree can be used in a multi-threaded program.

Every node has:
- A weak reference to a parent node (except for the root node, which has no parent).
- Strong references to zero or more child nodes.
- A value of a specific type.

Access to the tree and nodes is entirely through TreeNodeRef objects.

## Look at the source code

[main.rs](src/main.rs) - an example program.

[tree.rs](src/tree.rs) - the tree source code.

## Internal structure

The tree node structure looks like this:

    struct TreeNode<T> where T: Clone {
        pub parent: Option<TreeNodeWeak<T>>,
        pub children: Vec<TreeNodeRef<T>>,
        pub val: T,
    }

    #[derive(Clone)]
    pub struct TreeNodeRef<T> where T: Clone {
        inner: Arc<RwLock<TreeNode<T>>>,
    }

    pub struct TreeNodeWeak<T> where T: Clone {
        inner: Weak<RwLock<TreeNode<T>>>,
    }

- The parent reference is an Option, as the root node will have no parent.
- The parent reference is a Weak (not reference-counted) pointer, so that there is not a cyclic parent<->child reference, so that nodes drop from memory as expected when all their references are gone.
- The child references are stored in a Vec, to allow for a variable number of children.
- Every child or owned node reference is stored in an Arc (atomic reference-counted) pointer, so the nodes won't be dropped while they are still referred to by a parent, and access will be thread-safe.
- RwLock is used to allow for thread-safe access to the node data, so that we can't read from the node at the same time as writing to it.

Working with a node directly from the program would be rather messy, it would be working with a lot of
`Arc<RwLock<TreeNode<T>>>` objects. So instead of accessing the nodes directly, the are accessed through methods on a `TreeNodeRef` class, which abstracts over the complexity of the necessary operations.

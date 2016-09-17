use super::Result; // This is just a Result<T, TrieError>.

// A `NodeHandle` is how an in-memory node will refer to other nodes.
// This is slightly simplified.
enum NodeHandle {
	InMemory(usize), // a handle into the node allocator, which is a flat vector for cache locality,
	Hash(H256), // a hash, which can be used to fetch the node data on-demand from the backing database.
}

/// Raw representation of nodes.
#[derive(Clone, Eq, PartialEq, Debug)]
enum Node<'a> {
	/// Null trie node; could be an empty root or an empty branch entry.
	Empty,
	/// Leaf node; has key slice and value. Value may not be empty.
	Leaf(NibbleSlice<'a>, &'a [u8]),
	/// Extension node; has key slice and child node hash.
	Extension(NibbleSlice<'a>, &'a [u8]),
	/// Branch node; has array of 16 child nodes (each possibly empty) and an optional immediate node data.
	Branch([&'a [u8]; 16], Option<&'a [u8]>)
}

/// In-memory handles in Node types in the Trie.
#[derive(Debug)]
enum Node {
	/// Empty node.
	Empty,
	/// A leaf node contains the end of a key and a value.
	/// This key is encoded from a `NibbleSlice`, meaning it contains
	/// a flag indicating it is a leaf.
	Leaf(Vec<u8>, Vec<u8>),
	/// An extension contains a shared portion of a key and a child node.
	/// The shared portion is encoded from a `NibbleSlice` meaning it contains
	/// a flag indicating it is an extension.
	/// The child node is always a branch.
	Extension(Vec<u8>, NodeHandle),
	/// A branch has up to 16 children and an optional value.
	Branch(Box<[Option<NodeHandle>; 16]>, Option<Vec<u8>>)
}

/// A key-value datastore implemented as a database-backed modified Merkle tree.
pub trait TrieMut {
	/// Return the root of the trie.
	fn root(&mut self) -> &H256;

	/// Is the trie empty?
	fn is_empty(&self) -> bool;

	/// Does the trie contain a given key?
	fn contains(&self, key: &[u8]) -> Result<bool> {
		self.get(key).map(|x| x.is_some())
	}

	/// What is the value of the given key in this trie?
	fn get<'a, 'key>(&'a self, key: &'key [u8]) -> Result<Option<&'a [u8]>> where 'a: 'key;

	/// Insert a `key`/`value` pair into the trie. An empty value is equivalent to removing
	/// `key` from the trie.
	fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<()>;

	/// Remove a `key` from the trie. Equivalent to making it equal to the empty
	/// value.
	fn remove(&mut self, key: &[u8]) -> Result<()>;
}
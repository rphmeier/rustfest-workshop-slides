struct TrieDBMut<'db> {
	backing: &'db mut HashDB, // the backing database.
	...
}

impl<'db> TrieMut for TrieDBMut<'db> { ... }
.
struct SecTrieDBMut<'db> {
	inner: TrieDBMut<'db>,
}

impl<'db> TrieMut for SecTrieDBMut<'db> {
	...

	fn get<'a, 'key>(&'a self, key: &'key [u8]) -> Result<Option<&'a [u8]>> where 'a: 'key {
		let hash = key.sha3();
		self.inner.get(&hash);
	}

	fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
		let hash = key.sha3();
		self.inner.insert(&hash, value)
	}

	// you get the idea.
	...
}
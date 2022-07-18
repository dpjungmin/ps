/// Disjoint set data structure.
#[derive(Debug, Clone)]
pub struct DisjointSet {
    pub set: Vec<usize>,
    pub rank: Vec<usize>,
}

impl DisjointSet {
    /// Creates a new disjoint set with `n` sets.
    pub fn new(n: usize) -> Self {
        Self {
            set: Vec::from_iter(0..=n),
            rank: vec![1; n + 1],
        }
    }

    /// Returns the set of `u`.
    ///
    /// Panics if `u` is greater than `n` (see [`new`])
    ///
    /// [`new`]: #method.new
    pub fn find(&mut self, mut u: usize) -> usize {
        // path compression
        while u != self.set[u] {
            u = self.set[u];
        }

        u
    }

    /// Merges set `u` and `v`.
    ///
    /// Panics if `u` or `v` is greater than `n` (see [`new`])
    ///
    /// [`new`]: #method.new
    pub fn merge(&mut self, mut u: usize, mut v: usize) {
        u = self.find(u);
        v = self.find(v);

        if u == v {
            return;
        }

        // union by rank
        if self.rank[u] > self.rank[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.set[u] = v;

        if self.rank[u] == self.rank[v] {
            self.rank[v] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn new() {
        let n = 10;
        let ds = DisjointSet::new(n);

        for i in 0..=n {
            assert_eq!(ds.set[i], i);
        }

        assert!(ds.rank.iter().all(|&x| x == 1));
    }

    #[test]
    fn find() {
        let n = 5;
        let mut ds = DisjointSet::new(n);

        for i in 0..=n {
            assert_eq!(ds.find(i), i);
        }
    }

    #[test]
    fn merge() {
        let n = 5;
        let mut ds = DisjointSet::new(n);

        assert_ne!(ds.find(0), ds.find(1));
        ds.merge(0, 1);
        assert_eq!(ds.find(0), ds.find(1));

        assert_ne!(ds.find(2), ds.find(3));
        ds.merge(2, 3);
        assert_eq!(ds.find(2), ds.find(3));

        assert_ne!(ds.find(4), ds.find(5));
        ds.merge(4, 5);
        assert_eq!(ds.find(4), ds.find(5));

        assert_ne!(ds.find(1), ds.find(2));
        ds.merge(1, 2);
        assert_eq!(ds.find(0), ds.find(1));
        assert_eq!(ds.find(0), ds.find(2));
        assert_eq!(ds.find(0), ds.find(3));

        assert_ne!(ds.find(3), ds.find(4));
        ds.merge(3, 4);
        assert_eq!(ds.find(0), ds.find(1));
        assert_eq!(ds.find(0), ds.find(2));
        assert_eq!(ds.find(0), ds.find(3));
        assert_eq!(ds.find(0), ds.find(4));
        assert_eq!(ds.find(0), ds.find(5));
    }
}

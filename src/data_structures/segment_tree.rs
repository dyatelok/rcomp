pub mod segment_tree {
    pub struct SegmentTree<T: Copy + Clone, U: Copy + Clone> {
        data: Vec<T>,
        n: usize,
        size: usize,
        log: usize,
        combine_func: fn(T, T) -> T,
        update_func: fn(T, U) -> T,
        default: T,
    }

    impl<T: Copy + Clone, U: Copy + Clone> SegmentTree<T, U> {
        pub fn new(n: usize, combine_func: fn(T, T) -> T, update_func: fn(T, U) -> T, default: T) -> Self {
            let mut size = 1;
            let mut log = 0;
            while size < n {
                size <<= 1;
                log += 1;
            }
            let data = vec![default; 2 * size];
            Self {
                data,
                n,
                size,
                log,
                combine_func,
                update_func,
                default,
            }
        }

        pub fn update(&mut self, v: usize, x: U) {
            self.data[v + self.size] = (self.update_func)(self.data[v + self.size], x);
            for i in 1..=self.log {
                self.update_node(v >> i);
            }
        }

        pub fn get(&self, v: usize) -> T {
            self.data[v + self.size]
        }

        pub fn get_all(&self, l: usize, r: usize) -> T {
            let mut sml = self.default;
            let mut smr = self.default;
            let mut l = l + self.size;
            let mut r = r + self.size;
            while l < r {
                if l & 1 != 0 {
                    sml = (self.combine_func)(sml, self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = (self.combine_func)(self.data[r], smr);
                }
                l >>= 1;
                r >>= 1;
            }
            (self.combine_func)(sml, smr)
        }

        fn update_node(&mut self, k: usize) {
            self.data[k] = (self.combine_func)(self.data[2 * k], self.data[2 * k + 1]);
        }
    }
}

pub struct Subsets {
    n: usize,
    k: usize,
    curr_subset: Vec<usize>,
}

impl Subsets {
    pub fn new(n: usize, k: usize) -> Subsets {
        Subsets {
            n,
            k,
            curr_subset: (0..k).collect(),
        }
    }
}

/// Iterates over all k-subsets of {0,...,n - 1}.
/// # Examples
/// ```
/// use bipartite::combinatorics::Subsets;
/// let ten_choose_zero = Subsets::new(10, 0);
/// let ten_choose_five = Subsets::new(10, 5);
/// let ten_choose_eleven = Subsets::new(10, 11);
/// assert_eq!(1, ten_choose_zero.count());
/// assert_eq!(252, ten_choose_five.count());
/// assert_eq!(0, ten_choose_eleven.count());
/// ```
impl Iterator for Subsets {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k == 0 {
            self.k = self.n + 1;
            Some(Vec::new())
        } else if self.k > self.n
            || self.curr_subset[0] + self.k >= self.n + 1
        {
            None
        } else {
            let result_subset = self.curr_subset.clone();

            let mut idx = self.k - 1;

            while idx > 0 && self.curr_subset[idx] == self.n - self.k + idx {
                idx -= 1;
            }
            self.curr_subset[idx] += 1;
            while idx + 1 < self.k {
                self.curr_subset[idx + 1] = self.curr_subset[idx] + 1;
                idx += 1;
            }

            Some(result_subset)
        }
    }
}

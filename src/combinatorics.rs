/// Returns the value of n choose k.
/// # Examples
/// ```
/// use labisu::combinatorics::binomial;
/// assert_eq!(1, binomial(10, 0));
/// assert_eq!(10, binomial(10, 1));
/// assert_eq!(252, binomial(10, 5));
/// assert_eq!(1, binomial(10, 10));
/// assert_eq!(0, binomial(10, 11));
pub fn binomial(n: usize, k: usize) -> usize {
    if k > n { return 0 }
    let mut n = n;
    let mut k = std::cmp::min(k, n - k);
    let (mut acc_up, mut acc_down) = (1, 1);
    while k > 0 {
        acc_up *= n;
        acc_down *= k;
        n -= 1;
        k -= 1;
    }
    acc_up / acc_down
}
pub struct GraySubsets {
    n: usize,
    t: usize,
    i: usize,
    tau: Vec<usize>,
    pub g: Vec<usize>,
}

impl GraySubsets {
    pub fn new(n: usize, k: usize) -> GraySubsets {
        let mut tau: Vec<usize> = (0..n + 2).map(|x| x + 1).collect();
        tau[0] = if k > 0 { k + 1 } else { n + 1 };
        GraySubsets {
            n,
            t: k,
            i: 0,
            tau,
            g: (0..n + 1).map(|x| (x <= k) as usize).collect(),
        }
    }

    pub fn init(&self) -> Vec<usize> {
        (0..self.n).map(|x| (x < self.t) as usize).collect()
    }
}

impl Iterator for GraySubsets {
    type Item = (usize, usize);

    /// Implements the next method for the GraySubsets iterator.
    /// Returns transitions between subsets in the Gray code.
    /// The length for given n and k is min(0, n choose k minus 1) because
    /// it there is exactly one less element in transition sequence than in a given GrayCode 
    /// and for non-existent ones there are no transitions.
    /// # Examples
    /// ```
    /// use labisu::combinatorics::GraySubsets;
    /// use labisu::combinatorics::binomial;
    /// for n in 0..10 {
    ///    for k in 0..n + 2 {
    ///     let mut gray = GraySubsets::new(n, k);
    ///     let mut bin  = std::cmp::max(1, binomial(n, k)) - 1; 
    ///     let mut count = gray.into_iter().count();
    ///     assert_eq!(bin, count);
    ///     }
    /// }
    fn next(&mut self) -> Option<Self::Item> {
        let (change_0, change_1);

        if self.tau[0] < self.n + 1 {
            self.i           = self.tau[0];
            self.tau[0]      = self.tau[self.i];
            self.tau[self.i] = self.i + 1;

            if self.g[self.i] == 1 {
                let idx = if self.t > 0 { self.t } else { self.i - 1 };
                
                self.g[idx] = 1;
                self.g[self.i] = 0;

                change_1 = idx - 1;
                change_0 = self.i - 1;

                self.t += 1; 
            } else {
                let idx = if self.t > 1 { self.t - 1 } else { self.i - 1 };
                self.g[idx] = 0;
                self.g[self.i] = 1;
                change_0 = idx - 1;
                change_1 = self.i - 1;
                self.t -= 1;
            }

            if self.t  == self.i - 1 || self.t == 0 {
                self.t += 1;
            } else {
                self.t -= self.g[self.i - 1];
                self.tau[self.i - 1] = self.tau[0];

                self.tau[0] = if self.t == 0 { self.i - 1 } else { self.t + 1 }
            }

            return Some((change_0, change_1))
        }
        None
    }
}

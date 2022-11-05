/// Returns the value of binomial n choose k.
/// # Examples
/// ```
/// use bipartite::combinatorics::binomial;
/// assert_eq!(1, binomial(10, 0));
/// assert_eq!(252, binomial(10, 5));
/// assert_eq!(0, binomial(10, 11));
pub fn binomial(n: usize, k: usize) -> usize {
    match (n, k) {
        (_, 0) => 1,
        (0, _) => 0,
        (_, _) => (n * binomial(n - 1, k - 1)) / k,
    }
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
        let mut tau: Vec<usize> = (0..n + 1).map(|x| x + 1).collect();
        tau[0] = if k > 0 { k } else { n + 1 };
        GraySubsets {
            n,
            t: k,
            i: 0,
            tau,
            g: (0..n + 1).map(|x| if x < k { 1 } else { 0 }).collect(),
        }
    }

    pub fn init(&self) -> Vec<usize> {
        self.g.clone()
    }
}

impl Iterator for GraySubsets {
    type Item = [usize; 2];

    #[allow(clippy::never_loop)]
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = [0, 0];
        while self.tau[0] < self.n {
            self.i = self.tau[0];
            self.tau[0] = self.tau[self.i];
            self.tau[self.i] = self.i + 1;

            if self.g[self.i] == 1 {
                if self.t != 0 {
                    self.g[self.t - 1] = 1 - self.g[self.t - 1];
                    result[self.g[self.t - 1]] = self.t - 1;
                } else {
                    self.g[self.i - 1] = 1 - self.g[self.i - 1];
                    result[self.g[self.i - 1]] = self.i - 1;
                }
                self.t += 1;
            } else {
                if self.t != 1 {
                    self.g[self.t - 2] = 1 - self.g[self.t - 2];
                    result[self.g[self.t - 2]] = self.t - 2;
                } else {
                    self.g[self.i - 1] = 1 - self.g[self.i - 1];
                    result[self.g[self.i - 1]] = self.i - 1;
                }
                self.t -= 1;
            }

            self.g[self.i] = 1 - self.g[self.i];
            result[self.g[self.i]] = self.i;

            if self.t  == self.i || self.t == 0 {
                self.t += 1;
            } else {
                self.t -= self.g[self.i - 1];
                self.tau[self.i - 1] = self.tau[0];

                if self.t == 0 {
                    self.tau[0] = self.i - 1;
                } else {
                    self.tau[0] = self.t + 1;
                }
            }

            return Some(result)
        }
        None
    }
}

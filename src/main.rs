use bipartite::combinatorics::GraySubsets;

// The whole main function is only for testing purposes,
// it will be removed in the future.
fn main() {
    let n = 4;
    let k = 2;
    let gen = GraySubsets::new(n, k);
    let mut init = gen.init();
    print!("{:?}\n",  &init[0..n]);
    for change in gen {
        init[change[0]] = 0;
        init[change[1]] = 1;
        print!("{:?}\n", &init[0..n]);
        //print!("{:?}\n", change);
    }
}
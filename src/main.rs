use bipartite::combinatorics::GraySubsets;

fn main() {
    let gen = GraySubsets::new(4, 2);
    let mut init = gen.init();
    print!("{:?}\n",  &init[0..4]);
    for change in gen {
        init[change[0]] = 0;
        init[change[1]] = 1;
        print!("{:?}\n", &init[0..4]);
    }
}
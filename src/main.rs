mod disjoint;

fn main() {
    let mut ds = disjoint::DisjointSet::new(5);
    ds.union(0, 1);
    ds.union(2, 1);
    println!("{:?}", ds);
}

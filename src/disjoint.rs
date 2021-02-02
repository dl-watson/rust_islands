// Define a new data structure, with one (unnamed) field
// i32 is the default you should pick for a number, unless you have an argument for using a different type of number
#[derive(Debug)]
pub struct DisjointSet(Vec<Option<i32>>);

impl DisjointSet {
    pub fn new(size: usize) -> DisjointSet {
        // Create a vector (dynamic array), passing it the initial value and its size
        // None is the Rust equivalent of null
        let v = vec![None;size];
        DisjointSet(v)
    }

    // find is a method on a DisjointSet; it borrows the DisjointSet so that it can look at its contents
    // &self means that we're able to look at anything within this data structure (but we can't mutate it) -- read borrow
    pub fn find(&self, key: i32) -> i32 {
        let root = self.0[key as usize];
        match root {
            None => key,
            Some(index) => index
        }
    }

    // write borrow
    pub fn union(&mut self, key1: i32, key2: i32) {
        let root = self.find(key1);
        self.0[key2 as usize] = Some(root);
    }

}

/* 
Known bugs:

Trying to find a key value that's out of the vector's size scope will cause the program to CRASH

If everything is pointing to three, three will be None and everything else will be three
With union, you need to recursively find the root/identifier for the set ~ or you know what I mean
*/ 

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn find() {
        let ds = DisjointSet::new(5);
        assert_eq!(ds.find(0), 0);
    }

    #[test] 
    fn union() {
        let mut ds = DisjointSet::new(5);
        assert_ne!(ds.find(1), 0);
        ds.union(0, 1);
        assert_eq!(ds.find(1), 0);
    }
}

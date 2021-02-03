use std::cmp::{max, min};

// Define a new data structure, with one (unnamed) field
// i32 is the default you should pick for a number, unless you have an argument for using a different type of number
#[derive(Debug)]
pub struct DisjointSet(Vec<Option<i32>>);

impl DisjointSet {
    pub fn new(size: usize) -> DisjointSet {
        // Create a vector (dynamic array), passing it the initial value and its size
        // None is the Rust equivalent of null
        let constructor = vec![None;size];
        DisjointSet(constructor)
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
        let root1 = self.find(key1);
        let root2 = self.find(key2);

        // self.0[key2 as usize] = Some(root);

        let root1_is_identifier = root1 == key1;  
        let root2_is_identifier = root2 == key2;  

        // Three cases for union
        // Case 1: both of our numbers are their own identifiers
        if root1_is_identifier && root2_is_identifier {
            // set the root of the larger to be the root of the smaller 
            if root1 < root2 {
                self.0[root2 as usize] = Some(root1);
            } else {
                self.0[root1 as usize] = Some(root2);
            }
        }

        // Case 2: both aren't
        else if !root1_is_identifier && !root2_is_identifier {
            /*
            [None, Some(0), None, Some(2)]
            union(1, 3)
            -> [None, Some(0), Some(0), Some(0)]
            */
            // the winning identifier will be whichever one is smaller
            // the losing identifier now points to the winning 
            // all the references to the losing identifier will now point to the winning
            let winner = min(root1, root2);
            let loser = max(root1, root2);

            self.0[loser as usize] = Some(winner);
            // for option (nullable i32) in the data structure
            // mutably borrowed
            for opt in &mut self.0 {
                // match is a switch on steroids
                match opt {
                    // we only care about looking at references
                    // every non-null value is a reference in our data structure
                    Some(x) => {
                        // only set references to the losing value
                        // this x is an &mut i32 (& means reference, &mut means mutable reference)
                        if *x == loser {
                            // point them to the winning value
                            // at the location of x, set the value to winner
                            *x = winner
                        }
                    },
                    // if it's null, we don't care
                    _ => {}
                }
            }
        }
        
        // Case 3: one isn't an identifier
        else {
            let (identifier, not) = if root1_is_identifier {
                (root1, root2)
            } else {
                (root2, root1)
            };

            // set the one that isn't to the one that is
            self.0[identifier as usize] = Some(not);
        }
    }
}

pub struct SafeDisjointSet(DisjointSet);

impl SafeDisjointSet {
    pub fn new(size: usize) -> Self {
        Self(DisjointSet::new(size))
    }
    
    pub fn find(&self, key: i32) -> Option<i32> {
        if key as usize >= self.0.0.len() {
            None
        } else {
            Some(self.0.find(key))
        }
    }

    // pub fn union(&mut self, key1: i32, key2: i32) {

    // }
}
/* 
Known bugs:

Trying to find a key value that's out of the vector's size scope will cause the program to CRASH
*/ 

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn num_are_own_identifier() {
        let mut ds = DisjointSet::new(5);
        ds.union(0, 1);
        ds.union(4, 3);
        assert_eq!(ds.0, vec![None, Some(0), None, None, Some(3)]);
    }

    #[test]
    fn neither_are_identifier() {
        let mut ds = DisjointSet::new(5);
        ds.union(0,1 );
        ds.union(3, 4);
        assert_eq!(ds.0, vec![None, Some(0), None, None, Some(3)]);
        ds.union(1, 4);
        assert_eq!(ds.0, vec![None, Some(0), None, Some(0), Some(0)]);
    }

    #[test]
    fn one_isnt_an_identifier() {
        let mut ds = DisjointSet::new(5);
        ds.union(0,1 );
        ds.union(3, 4);
        assert_eq!(ds.0, vec![None, Some(0), None, None, Some(3)]);
        ds.union(1, 3);
        assert_eq!(ds.0, vec![None, Some(0), None, Some(0), Some(3)]);
    }
}

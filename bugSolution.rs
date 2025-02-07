fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 20;
} 
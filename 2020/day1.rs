fn main() {
    // define constants for the game input
    let expenses = [1721, 979, 366, 299, 675, 1456];
    let target = 2020;

    // our index pointers need to be scoped outside the loops
    // so we can access them after!
    let (mut i, mut j)  = (0, 0);

    // set a label so we can easily break out when we find the solution
    'outer: while i < expenses.len() {
        j = i + 1;
        while j < expenses.len()  {
            let x = expenses[i] + expenses[j];
            // check if this is the record combo we want
            if x == target {
                break 'outer;
            }
            j = j + 1;
        }
        i += 1;
    }

    // dont forget to multiply the values!
    println!("{}",  expenses[i] * expenses[j] );
}

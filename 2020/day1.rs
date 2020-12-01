fn main() {
    let expenses = [1721, 979, 366, 299, 675, 1456];
    let target = 2020;

    let (mut i, mut j)  = (0, 0);

    'outer: while i < expenses.len() {
        j = i + 1;
        while j < expenses.len()  {
            let x = expenses[i] + expenses[j];
            if x == target {
                break 'outer;
            }
            j = j + 1;
        }
        i += 1;
    }
    println!("{}",  expenses[i] * expenses[j] );
}

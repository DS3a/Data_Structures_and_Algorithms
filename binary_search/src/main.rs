/*
fn swap<T: Copy>(v: &mut Vec<T>, a: usize, b: usize) {
    if a > v.len() || b > v.len() {
        panic!("index needs to be within the length of the array");
    }
    let c: T = v[a];
    v[a] = v[b];
    v[b] = c;
}
*/

fn binary_search<T>(v: &Vec<T>, element_to_find: T) -> Result<usize, &str> 
where T: std::fmt::Debug + PartialEq + PartialOrd {
    println!("Current Array : {:?}", v);
    let size: usize = v.len();
    println!("The size is {}, we need to find the element {:?}", size, element_to_find);
    let mut min: usize = 0;
    let mut max: usize = size-1;
    let mut half: usize = (max+min)/2;

    loop {
        if v[half] == element_to_find {
            println!("found the element {:?} at index {}", element_to_find, half);
            return Result::Ok(half);
        } else if min >= max {
            println!("The element doesn't exist in the list");
            return Result::Err("Couldn't find element in the array");
        } else {
            if element_to_find > v[half] {
                min = half.clone();
            } else {
                max = half.clone();
            }
            half = (max+min)/2;
        }
    }
}

fn main() {
    let xs = vec![1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let element_to_find: i32 = 6;
/*    swap(&mut xs, 0, 100);
    println!("New Array : {:?}", xs);*/
    println!("{:?}", binary_search(&xs, element_to_find));
}

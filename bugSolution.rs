fn main() {    let mut vec = Vec::new();    vec.push(1);    vec.push(2);    let mut iter = vec.iter();    println!("{:?}", iter.next());    println!("{:?}", iter.next());    if let Some(value) = iter.next() {        println!("{:?}", value);    } else {        println!("Iterator exhausted");    }    //Alternative using a loop
    let mut vec2 = vec![1,2,3,4,5];
    for value in &vec2 {
        println!("{}", value);
    }
}
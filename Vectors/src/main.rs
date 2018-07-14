fn main() {
    let mut v: Vec<i32> = Vec::new();
    // type inference
    let v2 = vec![1, 2, 3];

    v.push(1);
    v.push(23);
    v.push(7);

    let mut var: &i32 = &v[2];
    let mut var: i32 = v[2];
    let third: Option<&i32> = v.get(2);

    println!("{:?}", third);

    let v = vec![1, 2, 3, 4, 5];

//    will cause a panic index out of bounds
//    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

//    will cause an error cannot borrow `v` as mutable because it is also borrowed as immutable
//    v.push(6);

    let v = vec![100, 32, 57];
//    iterating over immutable references to vector elements
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
//    iterating over mutable refs
    for i in &mut v {
        *i += 50;
    }


//    using enums to store different type values
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

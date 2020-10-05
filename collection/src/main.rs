fn main() {
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v = vec![1, 2, 3, 4, 5];

    // let _third: &i32 = &v[2];
    // let _third: Option<&i32> = v.get(2);

    // let _does_not_exist = &v[100];　実行がここで止まる
    // println!("{:?}", v.get(100)); //Noneが返る

    let first = v[0];

    v.push(6);

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    println!("{:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64, f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.0, 12.0),
    ];

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "inital contents".to_string();

    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); //sliceになるつまりstr
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    let s2 = 'l';
    s.push(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //&Stringを&strに型強制している
                       //&s2を&s2[..]スライスに参照外し型強制している
    println!("{}", s3);
    println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let len = String::from("Hola").len();

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in hello.chars() {
        println!("{}", c);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50, 40];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

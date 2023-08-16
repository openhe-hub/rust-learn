use std::collections::HashMap;

fn main() {
    let mut my_map: HashMap<String, i32> = HashMap::new();
    my_map.insert("aaa".to_string(), 100);
    my_map.insert("bbb".to_string(), 200);
    my_map.insert("ccc".to_string(), 300);
    println!("{:?}", my_map);
    match my_map.get(&String::from("aaa")) {
        Some(num) => println!("val = {}", num),
        None => println!("none"),
    }

    // insert
    my_map.entry(String::from("aaa")).or_insert(400);
    my_map.entry(String::from("ddd")).or_insert(400);

    // update
    my_map.insert("aaa".to_string(), 400);

    // remove
    my_map.remove(&String::from("aaa"));

    // loop k-v
    for (key, val) in &my_map {
        println!("key = {}, val = {}", key, val);
    }
}

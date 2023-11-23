

fn main() {
    let x: i64 = 5;

    // show x
    println!("x = {}", x);
    
    // create a rust list of oject
    let list = vec![1, 2, 3, 4, 5];

    // print the list
    println!("list = {:?}", list);

    // reate a backtright tree of object with
    // create a method to convert i32 to i64 
    let convert = |x: i32| -> i64 { x as i64 };

    // print the result of the method 
    println!("convert(32000) = {}", convert(32000));

    // create a method to to extract json data with three parameters
    // the first parameter is a reference to a string
    // the second parameter is a reference to a string
    // the third parameter is a reference to a string
    let extract = |x: &str, y: &str, z: &str| -> String { x.to_string() + y + z };

    // create a json data
    let json = r#"{"name": "John", "age": 30}"#;

    // convert the json data to string and extract the data
    //println!("Data = {}", extract(json));

    // create a method for backtoRight accept a 2 parameter X, Y, of is position add X on row else add . and Y on row and return a string converted tio Json
    let backtoRight = |x: i64, y: i64| -> String {
        let mut result = String::new();

        for i in 0..x {
            for j in 0..y {
                if i == j {
                    result.push_str("X");
                } else {
                    result.push_str(".");
                }
            }
            result.push_str("\r \n");
        }
        
        return result;
    };


    //print the backtoRight method
println!("backtoRight = {}", backtoRight(5, 5));

}

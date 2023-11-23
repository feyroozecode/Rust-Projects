pub fn var_test() {
    let mut x = 1;
    x = x + 2;
    let y: i32 = 5;
    let z: i32 = 12;

    // scope
    {
        println!(
            "Hello, the value of x is {} and the value of y is {} and z = {}",
            x, y, z
        );
    }

    assert_eq!(x, 3);

    println!("Hello, the value of x is {}, y {} ", x, y);

    // score two
    {
        let h = 21;
        let i = 10i32;
        let _j = 32i16; // to use after without warnings of compiler

        if h < i {
            println!("Of course h is little then i")
        } else {
            println!("h is bigger then i ")
        }
    }

    // scope three
    {
        let tab = &[0, 1, 2];

        println!("slices table = {:?}", tab);
        println!("show slices containe begin the index 2 => {:?}", &tab[1..]);
    }

    // vector
    {
        let mut v = Vec::new();

        v.push(0);
        v.push(3);
        v.push(7);

        let s = &v; // create a slices save the vector (v)

        println!("Slices from vectro v = {:?}", s);
    }
}

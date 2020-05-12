use std::ops::Add;

fn main() { // argc: isize, argv: c_void
    println!("Hello world!");

    // for Schleife
    count_up_to(3);

    // while Schleife
    let t = get_input_until("end\n", false);
    println!("{}", t);

    // Generics, Traits, Implementations
    print!("{}\n\n", generic_sum(&[1, 2, 3, 4]));
    print_traits();

    // struct match enum
    struct_match_enum();

}

fn print_traits() {
    struct Attributes {
        s1: String,
        s2: String
    }
    trait T {
        fn f(&self);
    }

    impl T for Attributes {
        fn f(&self) {
            print!("{}\t{}\n\n", self.s1, self.s2);
        }
    }

    let obj = Attributes {
        s1: String::from("first str"),
        s2: String::from("second str")
    };
    obj.f();
}

fn generic_sum<T: Add<Output = T> + Copy>(list: &[T]) -> T {
    let mut sum: T = list[0];
    for &t_elm in list[1..].iter() {
        sum = sum + t_elm;
    }
    return sum;
}

fn struct_match_enum() {
    struct S {
        u: i32,
        v: i32,
    }
    enum E {
        Action1 {x: i32, y: i32},
        Action2 {x: i32, y: i32, z: i32},
        Action3 {},
    }

    let u = 1;
    let s_imp = S {u, v: 2};

    let mut e_imp = E::Action3 {};
    match e_imp {
        E::Action1 {x, y} => {println!("{}, {}", x, y)},
        E::Action2 {x, y, z} => {println!("{}, {}, {}", x, y, z)},
        _ => {println!("Not Found!")},
    }

    e_imp = E::Action2 {x: s_imp.u, y: s_imp.v, z: 0};
    let z = match e_imp {
        E::Action1 {x, y} => {println!("{}, {}", x, y); -1},
        E::Action2 {x, y, z} => {println!("{}, {}", x, y); z},
        _ => {println!("Not Found!"); -1},
    };
    println!("Extracted z: {}.", z);

    e_imp = E::Action1 {x: s_imp.u, y: s_imp.v};
    match e_imp {
        E::Action1 {x, y} => {println!("{}, {}", x, y)},
        E::Action2 {x, y, z} => {println!("{}, {}, {}", x, y, z)},
        _ => {println!("Not Found!")},
    }
}

fn get_input_until(end: &str, print_steps: bool) -> String{
    let mut text: String = String::new(); // liest aktuelle Zeile
    let mut all: String = String::new(); // bildet den return String

    while &text != end { // Test, ob 'end' (und \n) eingegeben wurde
        all.push_str(&text); // letzte Zeile zu all hinzufügen
        text.clear(); // String leeren
        std::io::stdin().read_line(&mut text).expect("Zeile konnte nicht gelesen werden."); // Zeile lesen
        if print_steps {
            println!("{}", text);
        }
    }
    return all;
}

fn count_up_to(index: isize) {
    for i in 0..index {
        print!("{}\n", i);
    }
}
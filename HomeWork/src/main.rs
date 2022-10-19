use std::io;

fn main() {
    Homework1();
    Homework2();
}

fn Homework1() {
    let mut checkChildren = false;
    let org_arr: [i32; 8] = [1, 2,3,5,6,8, 10, 11];
    let sub_arr: [i32; 3] = [6,8,10];
    if org_arr.contains(&sub_arr[0]) {
        let mut a = 0;
        let x = org_arr.len()-sub_arr.len();
        while a < x {
            if org_arr[a] == sub_arr[0] {
                let mut b = 0;
                while b < sub_arr.len() {
                    if org_arr[a+b] == sub_arr[0+b] {
                        b+=1;
                        checkChildren = true;
                    } else {
                        checkChildren = false;
                        break;
                    }
                }
            }
            a+=1;
        }
    }

    if checkChildren {
        println!("org_arr contains sub_arr");
    } else {
        println!("org_arr does not contain sub_arr");
    }
}

fn Homework2() {
    let input = "adbcdaDd";
    let character = "a";
    let mut output = String::new();
    let mut count = 0;
    for char in input.chars() {
        if character.to_uppercase() == char.to_string().to_uppercase() {
            count += 1;
        } else {
            output.push(char)
        }
    }
    print!("{} ", count);
    print!("{}", output);
}

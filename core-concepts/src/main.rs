fn main() {

    let x = 5 / 2;

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.1;// or x
    let one = tup.2; // or z

    another_function(5);
    print_labeled_measurement(5, 'h');

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // isso aqui é doido, eles tem que ser sempre do mesmo tipo
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn loop_example(){
     let mut counter = 0;

     // poder retornar valor de um loop é bem util até
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


fn for_in_example () {
    let a = [10, 20, 30, 40, 50];

    // isso é top demais
    for element in a {
        println!("the value is: {element}");
    }
}

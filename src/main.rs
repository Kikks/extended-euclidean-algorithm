use std::io;

/////////////// Basic Euclidean Algorithm
/// From the basic division algorithm, When we divide an integer from a non-zero integer,
/// there exists integers q and r such that:
///     a = bq + r where  0 <= r <= b
/// where q is the quotient and r is the remainder.
/// The Euclidean Algorithm applies the division algorithm repeatedly to find the
/// GCD (Greatest Common Divisor) of integers a and b. The divisor is repeatedly divided
/// by the remainder until the remainder is zero. The last non-zero remainder is the
/// greatest common divisor.
//////////////////////////////////////////////////////////////////////////////////////

/////////////// Extended Euclidean Algorithm
/// Asides helping to compute the GCD (Greatest Common Divisor) of two numbers, it also
/// helps find integers x and y such that:
///     ax + by = gcd(a,b)
/// From the Bezout's Identity we can guaranty that x & y exist.
//////////////////////////////////////////////////////////////////////////////////////

/// A function that implements the extended Euclidean algorithm. The function accepts two numbers:
/// - The first number is the number whose inverse is to be found (a)
/// - The modulo in which the operation will be carried out (b)
/// The function returns a tuple with three values:
/// - The GCD of a and b
/// - x (As explained in the extended Euclidean Algorithm above)
/// - y (As explained in the extended Euclidean Algorithm above)
fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    // This is the base case: when b = 0,
    //     gcd(a, 0) = a
    // Hence the Euclidean equation becomes:
    //     a(1) + b(0) = a
    if b == 0 {
        return (a, 1, 0);
    }

    // Recursively call the extended Euclidean Algorithm
    let (gcd, x1, y1) = extended_euclidean_algorithm(b, a % b);

    // Compute x and y by working backwards the Euclidean Algorithm
    let x = y1;
    let y = x1 - (a / b) * y1;

    // Return the tuple
    return (gcd, x, y);
}

/////////////// Inverses
/// The inverse of a (mod b) exists only if a and b are coprime; i.e gcd(a, b) = 1.
/// Using the extended Euclidean Algorithm,
///     ax + by = 1
/// Taking the modulo of both sides, we have:
///     ax + by ≅ 1 (mod b)
/// "b(y) mod (b)"" will always be 0 for an integer y. Therefore:
///     ax ≅ 1 (mod b)
/// Hence, x becomes the multiplicative inverse of a (mod b)
//////////////////////////////////////////////////////////////////////////////////////
fn get_inverse(number: i32, modulo: i32) -> String {
    // Get the GCD, x and y from the extended Euclidean Algorithm
    let (gcd, x, _y) = extended_euclidean_algorithm(number, modulo);

    // If GCD is not equal to 1 then, the inverse of the number does not exist.
    if gcd != 1 {
        return format!("{} does not have an inverse in modulo {}", number, modulo);
    } else {
        // Return the inverse of the number to the user
        return format!(
            "The inverse of {} mod {} is: {}",
            number,
            modulo,
            if x < 0 { x + modulo } else { x }
        );
    }
}

// A function to get an input as a number from the user with a customizable message
// and return the inputted number.
fn get_user_input(question: String) -> i32 {
    let mut input_line = String::new();
    println!("{}", question);
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let x: i32 = input_line.trim().parse().expect("Input not an integer");

    return x;
}

fn main() {
    // Request a number whose inverse is to be found from the user
    let number = get_user_input(String::from(
        "What number would you like to the find the inverse of? ",
    ));

    // Request the modulo the operation will be carried out in
    let modulo = get_user_input(String::from("What modulo are you working in? "));

    // Get the inverse of the number
    let inverse = get_inverse(number, modulo);

    // Print out the inverse to the standard output!
    println!("{}", inverse);
}

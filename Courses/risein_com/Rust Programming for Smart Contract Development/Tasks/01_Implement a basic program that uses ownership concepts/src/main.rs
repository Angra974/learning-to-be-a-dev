
/// Returns a string created by concatening 2 strings
/// passed as arguments
///
/// # Arguments
///
/// `s1` - A string slice that holds a string value
/// `s2` - A string slice that holds a string value
///
/// # Examples
///
/// ```
///     let s1 = "Hello";
///     let s2 = " John";
///
///     let concatened_string = concatenate_strings(s1, s2);
///
///     assert_eq!(concatenated_string, "Hello John") // true
///
/// ```
fn concatenate_strings ( s1: &str, s2: &str) -> String {

    let mut result: String = s1.into();

    result.push_str(s2);
	
    result
}

fn main() {

    let string1: &str = "Hello";
    let string2: &str = " World!";

    let concatenate_string = concatenate_strings(&string1, &string2);

	
    println!("{concatenate_string}");


}

#[test]
fn hello_john () {
    let string1 = "Hello";
    let string2 = " John";
    let actual_input = "Hello John";
    let expected_output = concatenate_strings(&string1, &string2);

    assert_eq!(actual_input,expected_output);
}

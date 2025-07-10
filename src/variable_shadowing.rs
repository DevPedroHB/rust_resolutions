pub fn variable_shadowing() {
  // Exampleo 01

  let n = 5;

  {
    let n = 6;

    println!("The value of n in the inner scope is: {n}");
  }

  println!("The value of n is: {n}");

  // Example 02

  let spaces = "      ";
  let spaces = spaces.len();

  println!("The length of spaces is: {spaces}");
}

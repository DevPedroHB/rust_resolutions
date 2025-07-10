const PI: f64 = 3.1415;

pub fn variables() {
  // Variables mutable
  let mut number = 10;

  println!("Number: {}", number);

  number = 20;

  println!("Number: {}", number);

  // Constants

  const ONE_MINUTE: i32 = 60;
  const ONE_HOUR: i32 = ONE_MINUTE * 60;

  println!("One minute is: {ONE_MINUTE}s");
  println!("One hour is: {ONE_HOUR}s");

  println!("PI is: {PI}");
}

pub fn solution() -> String {
  let mut fib1 = 1;
  let mut fib2 = 2;
  let mut sum = 0;

  while fib2 < 4_000_000 {
    if fib2 % 2 == 0 {
      sum += fib2;
    }
    let new = fib1 + fib2;
    fib1 = fib2;
    fib2 = new;
  }

  format!("{sum}")
}
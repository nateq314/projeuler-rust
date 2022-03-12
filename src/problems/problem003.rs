pub fn solution() -> String {
  let original: u64 = 600_851_475_143;
  let mut n = original;
  let mut divisors: Vec<u64> = vec![];

  while n != 1 {
    let limit = (n as f64).sqrt().floor() as u64;
    for maybe_divisor in 2..=limit {
      if n % maybe_divisor == 0 {
        divisors.push(maybe_divisor);
        n /= maybe_divisor;
      }
    }
  }
  

  format!("{:?}", divisors)
}

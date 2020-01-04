use std::collections::HashMap;

fn decomp(n: i32) -> String {
  let (
    mut primes,
    mut primes_map,
    mut decomp_str
  ) = (
    vec![2],
    HashMap::new(),
    vec![]
  );

  if n == 1 {
    return "1".to_owned();
  }

  for i in 1..n + 1 {
    let (factors, new_prime) = parse(i, &primes);
    // add new prime to vector for next usage
    if new_prime > 1 {
      primes.push(new_prime);
    }
    for (factor, times) in factors {
      if let Some(previous_times) = primes_map.get_mut(&factor) {
        *previous_times = *previous_times + times;
      } else {
        primes_map.insert(factor, times);
      }
    }
  }

  for prime in primes {
    if let Some(times) = primes_map.get(&prime) {
      let mut factor_str = prime.to_string();
      if *times > 1 {
        factor_str = factor_str + "^" + &times.to_string();
      }
      decomp_str.push(factor_str);
    }
  }

  decomp_str.join(" * ").to_owned()
}

fn parse(mut m: i32, primes: &Vec<i32>) -> (HashMap<i32, i32>, i32) {
  let mut factors = HashMap::new();
  if m == 1 {
    return (factors, m);
  } else {
    for prime in primes {
      while m % prime == 0 {
        m = m / prime;
        if let Some(i) = factors.get_mut(prime) {
          *i += 1;
        } else {
          factors.insert(*prime, 1);
        }
      }
    }
    // new prime
    if m > 1 {
      factors.insert(m, 1);
    }
  }
  (factors, m)
}
   
fn dotest(n: i32, exp: &str) -> () {
  println!("n:{:?}", n);
  let ans = decomp(n);
  println!("actual: {:?}", ans);
  println!("expect: {:?}", exp.to_string());
  println!("{}", ans == exp.to_string());
  assert_eq!(ans, exp.to_string());
  println!("{}", "-");
}

pub fn test() {
  dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
  dotest(5, "2^3 * 3 * 5");
  dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
  dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
  dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
}

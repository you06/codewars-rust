fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
  let mut r = vec![];

  for i in m..(n + 1) {
    let sqrt_sum: u64 = get_divisors(i).into_iter().map(|x| x * x).sum();
    let root = (sqrt_sum as f64).sqrt() as u64;
    if root * root == sqrt_sum {
      r.push((i, sqrt_sum));
    }
  }

  r
}

fn get_divisors(i: u64) -> Vec<u64> {
  let mut divisors = vec![];
  for d in 1..((i as f64).sqrt() as u64 + 1) {
    let q = i / d;
    if d * q == i {
      divisors.push(d);
      if d != q {
        divisors.push(q);
      }
    }
  }
  divisors
}

fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
  assert_eq!(list_squared(m, n), exp)
}

pub fn test() {
  testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
  testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
  testing(42, 250, vec![(42, 2500), (246, 84100)]);
  testing(300, 600, vec![]);
}

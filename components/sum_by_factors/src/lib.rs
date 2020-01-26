use std::collections::HashSet;

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
  let (
    factors,
    mut primes,
    mut v
  ): (
    Vec<(HashSet<i64>, i64)>,
    HashSet<i64>,
    Vec<(i64, i64)>
  ) = (
    l.into_iter().map(|n| (get_factors(n), n)).collect(),
    HashSet::new(),
    Vec::new()
  );

  for (factor, _) in &factors {
    for f in factor {
      primes.insert(*f);
    }
  }

  for prime in primes {
    v.push((
      prime,
      factors.clone().into_iter()
        .filter(|(f, _)| !f.get(&prime).is_none())
        .map(|(_, n)| n).sum()
    ))
  }

  v.sort_by(|(a, _), (b, _)| a.cmp(b));
  v
}

fn get_factors(n: i64) -> HashSet<i64> {
  let (
    s,
    mut p,
    mut f
  ) = (
    (n.abs() as f64).sqrt() as i64,
    n.abs(),
    HashSet::new()
  );

  for i in 2..s + 1 {
    if p % i == 0 {
      f.insert(i);
      while p % i == 0 {
        p = p / i;
      }
    }
  }
  if p != 1 {
    f.insert(p);
  }

  f
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
  assert_eq!(sum_of_divided(l), exp)
}

pub fn test() {
  testing(vec![12, -15], vec![ (2, 12), (3, -3), (5, -15) ]);
  testing(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
  testing(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
}

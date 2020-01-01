static GOOD: &'static [u32] = &[1, 2, 3, 3, 4, 10];
static EVIL: &'static [u32] = &[1, 2, 2, 2, 3, 5, 10];

fn good_vs_evil(good: &str, evil: &str) -> String {
  let good_val: Vec<u32> = good.split(' ')
                  .map(|x| x.parse().unwrap())
                  .collect();
  let evil_val: Vec<u32> = evil.split(' ')
                  .map(|x| x.parse().unwrap())
                  .collect();
  let (mut good_sum, mut evil_sum): (u32, u32) = (0, 0);
  for i in 0..good_val.len() {
    good_sum += good_val[i] * GOOD[i]
  }
  for i in 0..evil_val.len() {
    evil_sum += evil_val[i] * EVIL[i]
  }

  if good_sum > evil_sum {
    return "Battle Result: Good triumphs over Evil".to_owned();
  } else if good_sum < evil_sum {
    return "Battle Result: Evil eradicates all trace of Good".to_owned();
  }
  "Battle Result: No victor on this battle field".to_owned()
}

pub fn test() {
  assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
  assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
  assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

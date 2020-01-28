fn dbl_linear(n: u32) -> u32 {
  let (
    mut m2,
    mut m3,
    mut v
  ): (usize, usize, Vec<u32>) = (
    0,
    0,
    vec![1]
  );

  for _ in 0..n {
    let (v2, v3) = (
      v[m2] * 2 + 1,
      v[m3] * 3 + 1
    );

    if v2 < v3 {
      m2 = m2 + 1;
      v.push(v2);
    } else if v2 > v3 {
      m3 = m3 + 1;
      v.push(v3);
    } else {
      m2 = m2 + 1;
      m3 = m3 + 1;
      v.push(v2);
    }
  }

  *v.last().unwrap()
}

fn testing(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

pub fn test() {
  testing(0, 1);
  testing(1, 3);
  testing(2, 4);
  testing(3, 7);
  testing(10, 22);
  testing(20, 57);
  testing(30, 91);
  testing(50, 175);
  testing(100, 447);
  testing(1000, 8488);
  testing(3000, 34510);
  testing(11111, 177256);
  testing(22222, 441151);
  testing(33333, 714016);
  testing(44444, 1027893);
  testing(60000, 1511311);
  testing(180000, 6100623);
}

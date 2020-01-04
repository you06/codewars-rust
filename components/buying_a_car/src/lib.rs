fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
  let (
    mut de_new,
    mut de_old,
    mut money,
    mut month
  ) = (
    new as f64,
    old as f64,
    old as f64,
    0
  );

  while money < de_new {
    month += 1;
    let de: f64 = 1.0 - (perc + 0.5 * (month / 2) as f64) / 100.0;
    de_old = de_old * de;
    de_new = de_new * de;
    money = de_old + (month * saving) as f64;
  }
  (month, (money - de_new).round() as i32)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
  assert_eq!(nb_months(old, new, saving, perc), exp)
}

pub fn test() {
  testing(2000, 8000, 1000, 1.5, (6, 766));
  testing(12000, 8000, 1000, 1.5 , (0, 4000));
  testing(8000, 12000, 500, 1.0, (8, 597));
  testing(18000, 32000, 1500, 1.25, (8, 332));
  testing(7500, 32000, 300, 1.55, (25, 122));
}

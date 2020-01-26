fn next_smaller_number(n: u64) -> Option<u64> {
  let (
    mut chars,
    mut top_chars
  ): (
    Vec<char>,
    Vec<char>
  ) = (
    n.to_string().chars().collect(),
    Vec::new()
  );

  chars.reverse();
  
  for &c in &chars {
    if top_chars.iter().all(|x| x >= &c) {
      top_chars.push(c);
    } else {
      if let Some(m) = top_chars.clone().iter().filter(|&x| x < &c).max_by(|a, b| a.cmp(b)) {
        if *m == '0' && top_chars.len() == chars.len() - 1 {
          return None;
        }
        let index = top_chars.clone().into_iter().position(|e| e == *m).unwrap();
        top_chars.remove(index);
        top_chars.push(c);
        top_chars.sort();
        top_chars.push(*m);
        top_chars.append(&mut chars.clone().split_off(top_chars.len()));
        return Some(top_chars.into_iter().rev().collect::<String>().parse().unwrap());
      } else {
        return None;
      }
    }
  }

  None
}


pub fn test() {
  assert_eq!(None, next_smaller_number(2));
  assert_eq!(Some(123), next_smaller_number(132));
  assert_eq!(Some(1231), next_smaller_number(1312));
  assert_eq!(Some(2017), next_smaller_number(2071));

  assert_eq!(Some(12), next_smaller_number(21));
  assert_eq!(Some(790), next_smaller_number(907));
  assert_eq!(Some(513), next_smaller_number(531));
  assert_eq!(Some(11721), next_smaller_number(12117));
  assert_eq!(None, next_smaller_number(1027));
  assert_eq!(Some(414), next_smaller_number(441));
}

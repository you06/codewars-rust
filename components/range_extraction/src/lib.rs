enum Item {
  Range(i32, i32),
  Point(i32),
}

fn range_extraction(a: &[i32]) -> String {
  if a.len() < 3 {
    return a.into_iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
  }

  let mut range: Vec<Item> = Vec::new();

  for &i in a {
    if range.len() < 2 {
      range.push(Item::Point(i));
      continue;
    }

    if let Some(last) = range.last_mut() {
      if let Item::Range(_, end) = last {
        if *end + 1 == i {
          *end = *end + 1;
        } else {
          range.push(Item::Point(i))
        }
        continue;
      }
    }

    let len = range.len();
    if let Item::Point(p1) = range[len - 2] {
      if let Item::Point(p2) = range[len - 1] {
        if p1 + 1 == p2 && p2 + 1 == i {
          range.pop();
          range.pop();
          range.push(Item::Range(p1, i));
          continue;
        }
      }
    }

    range.push(Item::Point(i))
  }

  range.into_iter().map(|x| match x {
    Item::Point(i) => i.to_string(),
    Item::Range(s, e) => format!("{}-{}", s, e),
  }).collect::<Vec<String>>().join(",")
}

pub fn test() {
  assert_eq!("-6,-3", range_extraction(&[-6,-3]));	
  assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
  assert_eq!("-3--1,2,10,15,16,18-20", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
}

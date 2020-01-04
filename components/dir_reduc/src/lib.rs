#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {NORTH, SOUTH, EAST, WEST}


fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
  use Direction::*;

  fn oppo(i: Direction) -> Direction {
    match i {
      NORTH => SOUTH,
      SOUTH => NORTH,
      EAST => WEST,
      WEST => EAST,
    }
  }

  let mut res: Vec<Direction> = Vec::new();

  for &d in arr {
    let t = oppo(d);

    match res.last() {
      Some(r) => {
        if *r == t {
          res.pop();
        } else {
          res.push(d);
        }
      },
      None => res.push(d),
    }
  }

  res 
}

pub fn test() {
  use Direction::*;
  let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
  assert_eq!(dir_reduc(&a), [WEST]);
  let a = [NORTH, WEST, SOUTH, EAST];
  assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
}

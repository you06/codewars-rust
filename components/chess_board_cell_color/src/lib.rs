fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
  get_color(cell1) == get_color(cell2)
}

fn get_color(cell: &str) -> bool {
  let sum: u8 = cell.chars()
    .map(|x| x as u8)
    .sum();
  sum % 2 == 0
}

pub fn test() {
  assert_eq!(chessboard_cell_color("A1", "C3"), true);
  assert_eq!(chessboard_cell_color("A1", "H3"), false);
  assert_eq!(chessboard_cell_color("A1", "A2"), false);
  assert_eq!(chessboard_cell_color("A1", "C1"), true);
  assert_eq!(chessboard_cell_color("A1", "A1"), true);
}

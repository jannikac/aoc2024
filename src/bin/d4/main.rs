const INPUT: &str = include_str!("input.txt");
const TEST: &str = include_str!("test.txt");

const SEARCH: &str = "XMAS";

enum DirectionSign {
    Positive,
    Negative,
}

enum Direction {
    Horizontal(usize, usize, DirectionSign),
    Vertical(usize, usize, DirectionSign),
    Diagonal(usize, usize, DirectionSign, DirectionSign),
}

/// returns a string from the search phrase in a specified direction
fn get_direction_from(direction: Direction, matrix: &Vec<Vec<char>>) -> bool {
    let res = match direction {
        Direction::Horizontal(row, col, dir) => {
            let res = match dir {
                DirectionSign::Positive => (0..SEARCH.chars().count())
                    .map(|v| matrix[row].get(col + v))
                    .collect::<Option<String>>(),
                DirectionSign::Negative => (0..SEARCH.chars().count())
                    .map(|v| matrix[row].get(col.checked_sub(v)?))
                    .collect::<Option<String>>(),
            };
            res
        }
        Direction::Vertical(row, col, dir) => {
            let res = match dir {
                DirectionSign::Positive => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row + v)?.get(col))
                    .collect::<Option<String>>(),
                DirectionSign::Negative => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row.checked_sub(v)?)?.get(col))
                    .collect::<Option<String>>(),
            };
            res
        }
        Direction::Diagonal(row, col, dir_row, dir_col) => {
            let res = match (dir_row, dir_col) {
                (DirectionSign::Positive, DirectionSign::Positive) => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row + v)?.get(col + v))
                    .collect::<Option<String>>(),
                (DirectionSign::Positive, DirectionSign::Negative) => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row + v)?.get(col.checked_sub(v)?))
                    .collect::<Option<String>>(),
                (DirectionSign::Negative, DirectionSign::Positive) => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row.checked_sub(v)?)?.get(col + v))
                    .collect::<Option<String>>(),
                (DirectionSign::Negative, DirectionSign::Negative) => (0..SEARCH.chars().count())
                    .map(|v| matrix.get(row.checked_sub(v)?)?.get(col.checked_sub(v)?))
                    .collect::<Option<String>>(),
            };
            res
        }
    };

    if let Some(x) = res {
        x == SEARCH
    } else {
        false
    }
}

fn part_1() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum_of_xmas = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let checks = [
                get_direction_from(
                    Direction::Horizontal(row, col, DirectionSign::Positive),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Horizontal(row, col, DirectionSign::Negative),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Vertical(row, col, DirectionSign::Positive),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Vertical(row, col, DirectionSign::Negative),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Diagonal(row, col, DirectionSign::Positive, DirectionSign::Positive),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Diagonal(row, col, DirectionSign::Positive, DirectionSign::Negative),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Diagonal(row, col, DirectionSign::Negative, DirectionSign::Positive),
                    &matrix,
                ),
                get_direction_from(
                    Direction::Diagonal(row, col, DirectionSign::Negative, DirectionSign::Negative),
                    &matrix,
                ),
            ];
            let found_search_word = checks.iter().filter(|v| **v).count();
            sum_of_xmas = sum_of_xmas + found_search_word;
        }
    }
    dbg!(sum_of_xmas);
}

enum DirKind {
    DirA(usize, usize),
    DirB(usize, usize),
}

fn part_2_find(dir: DirKind, matrix: &Vec<Vec<char>>) -> Option<String> {
    let res = match dir {
        DirKind::DirA(row, col) => [
            matrix.get(row.checked_sub(1)?)?.get(col.checked_sub(1)?)?,
            &matrix[row][col],
            matrix.get(row + 1)?.get(col + 1)?,
        ]
        .into_iter()
        .collect::<String>(),
        DirKind::DirB(row, col) => [
            matrix.get(row.checked_sub(1)?)?.get(col + 1)?,
            &matrix[row][col],
            matrix.get(row + 1)?.get(col.checked_sub(1)?)?,
        ]
        .into_iter()
        .collect::<String>(),
    };

    Some(res)
}

fn part_2() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum_of_real_xmas = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let res1 = part_2_find(DirKind::DirA(row, col), &matrix);
            let res2 = part_2_find(DirKind::DirB(row, col), &matrix);
            if let (Some(a), Some(b)) = (res1, res2) {
                if (a == "MAS" || a == "SAM") && (b == "MAS" || b == "SAM") {
                    sum_of_real_xmas = sum_of_real_xmas + 1;
                }
            }
        }
    }
    dbg!(sum_of_real_xmas);
}

fn main() {
    part_1();
    part_2();
}

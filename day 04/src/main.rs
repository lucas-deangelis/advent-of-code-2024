use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

fn file_to_grid(filename: String) -> Vec<Vec<char>> {
    let file = std::fs::read_to_string(filename).expect("Failed to read input file");
    let lines = file.lines().collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        grid.push(line.chars().collect::<Vec<char>>())
    }

    grid
}

struct Grid {
    vec: Vec<Vec<char>>,
}

impl Grid {
    fn from_file(filename: String) -> Self {
        Self { vec: file_to_grid(filename) }
    }

    fn count_xmas(&self) -> u64 {
        // Iterate on all the points and count from them?
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vec {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}


fn silver_star(s: &str) -> u64 {
    return 0
}


#[test]
fn test_safe_silver() {
    let input = Grid::from_file("input.example.txt".to_string());
    insta::assert_snapshot!(input, @r"
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    ");
}

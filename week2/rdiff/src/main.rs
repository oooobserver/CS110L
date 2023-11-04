use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut res = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        res.push(line_str);
    }   
    Ok(res)
}


fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let mut grid = Grid::new(seq1.len()+1, seq2.len()+1);
    let (row,column) = grid.size();
    for i in 0..row{
        for j in 0..column{
            grid.set(i,j,0).unwrap();
        }
    }

    for i in 0..row-1{
        for j in 0..column-1{
            if seq1[i] == seq2[j]{
                grid.set(i+1, j+1, grid.get(i, j).unwrap()+1).unwrap();
            }else{
                grid.set(i+1, j+1, grid.get(i+1, j).unwrap().max(grid.get(i, j+1).unwrap())).unwrap();
            }
        }
    }
        
    grid
}

fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    
    if i > 0 && j > 0 && lines1[i-1] == lines2[j-1]{
        print_diff(lcs_table, lines1, lines2, i-1, j-1);
        println!( "  {}",lines1[i-1]);
    }else if j > 0 && (i == 0 || lcs_table.get(i,j-1) >= lcs_table.get(i-1,j)){
        print_diff(lcs_table, lines1, lines2,  i, j-1);
        println!("> {}", lines2[j-1]);
    }else if i > 0 && (j == 0 || lcs_table.get(i,j-1) < lcs_table.get(i-1,j)) {
        print_diff(lcs_table, lines1, lines2, i-1, j);
        println!( "< {}",lines1[i-1]);
    }else{
        println!(" ");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let v1 = read_file_lines(filename1).expect("read failed");
    let v2 = read_file_lines(filename2).expect("read failed");

    let grid = lcs(&v1, &v2);

   print_diff(&grid, &v1, &v2, v1.len(), v2.len());

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}

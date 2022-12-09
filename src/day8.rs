use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
pub struct Map {
    number_of_cols: usize,
    number_of_rows: usize,
    trees: Vec<u8>
}

impl Map {
    
    /*
        Count the number of trees on the outside rim. Those are always visible
    */
    pub fn count_outer_rim(&self) -> Result<usize, &'static str> {
        if self.number_of_cols == 0 && self.number_of_rows == 0 {
            Err("Cannot calculate the outer rim")
        }
        else {
            Ok(
                self.number_of_cols * 2 + 
                (self.number_of_rows - 2) * 2)
        }
    }

    fn row_col_to_index(&self, row: usize, col: usize) -> usize {
        ((row-1) * self.number_of_cols) + (col-1)
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.number_of_cols == other.number_of_cols && self.number_of_rows == other.number_of_rows
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Option<Map> {
    Some(
        Map {
            number_of_rows: input
            .lines()
            .count(),
            number_of_cols: input.lines().next().unwrap().len(),
            trees: input.lines().flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect()
        }
    )
}


#[aoc(day8, part1)]
pub fn part1(input: &Map) -> Option<usize> {

    let mut sum_trees_visible: usize = 0;

    for row in 2..input.number_of_rows {
        for col in 2..input.number_of_cols {
            let tree = input.trees.get(input.row_col_to_index(row, col)).unwrap();

            let mut visible_left = true;
            let mut visible_right = true;
            let mut visible_bottom = true;
            let mut visible_top = true;

            for left_index in input.row_col_to_index(row, 1)..input.row_col_to_index(row, col) {
                if input.trees.get(left_index).unwrap() >= tree {
                    visible_left = false; break;
                }
            };
            for right_index in input.row_col_to_index(row, col)+1..=input.row_col_to_index(row, input.number_of_cols) {
                if input.trees.get(right_index).unwrap() >= tree {
                    visible_right = false; break;
                }
            };
            for top_index in 1..row {
                if input.trees.get(input.row_col_to_index(top_index, col)).unwrap() >= tree {
                    visible_top = false; break;
                }
            }
            for bottom_index in row+1..=input.number_of_rows {
                if input.trees.get(input.row_col_to_index(bottom_index, col)).unwrap() >= tree {
                    visible_bottom = false; break;
                }
            }
            if visible_left || visible_right || visible_top || visible_bottom { sum_trees_visible += 1};
        }
    }

    Some(input.count_outer_rim().ok().unwrap() + sum_trees_visible)
}

#[aoc(day8, part2)]
pub fn part2(input: &Map) -> Option<usize> {

    let mut biggest_scenic_score: usize = 0;

    for row in 2..input.number_of_rows {
        for col in 2..input.number_of_cols {
            let tree = input.trees.get(input.row_col_to_index(row, col)).unwrap();
            let mut viewing_disance_right = 0;
            let mut viewing_disance_left = 0;
            let mut viewing_disance_top = 0;
            let mut viewing_disance_bottom = 0;

            for left_index in (input.row_col_to_index(row, 1)..input.row_col_to_index(row, col)).rev() {
                println!("{}:{}: {} - {}", row, col, left_index, input.trees.get(left_index).unwrap());
                viewing_disance_left+=1;
                if input.trees.get(left_index).unwrap() >= tree {
                    break;
                }
            };

            for right_index in input.row_col_to_index(row, col)+1..=input.row_col_to_index(row, input.number_of_cols) {
                viewing_disance_right+=1;
                if input.trees.get(right_index).unwrap() >= tree {
                    break;
                }
            };
            for top_index in (1..row).rev() {
                viewing_disance_top+=1;
                if input.trees.get(input.row_col_to_index(top_index, col)).unwrap() >= tree {
                    break;
                }
            }
            for bottom_index in row+1..=input.number_of_rows {
                viewing_disance_bottom+=1;
                if input.trees.get(input.row_col_to_index(bottom_index, col)).unwrap() >= tree {
                    break;
                }
            }
            println!("{}({},{}): {} ({} * {} * {} * {})", tree, row, col, viewing_disance_right * viewing_disance_left * viewing_disance_top * viewing_disance_bottom,  viewing_disance_right , viewing_disance_left , viewing_disance_top , viewing_disance_bottom );

            if viewing_disance_right * viewing_disance_left * viewing_disance_top * viewing_disance_bottom >= biggest_scenic_score { biggest_scenic_score =  viewing_disance_right * viewing_disance_left * viewing_disance_top * viewing_disance_bottom; };
        }
    }

    Some(biggest_scenic_score)

}


#[cfg(test)]
mod tests {

    use super::{input_generator, Map, part1, part2};

    #[test]
    fn test_map_coord_to_index() {
        
        // given that cols = 5 and rows = 5
        // 1, 1 ==> 0
        // 1, 2 ==> 1
        // 1, 3 ==> 2
        // 1, 4 ==> 3
        // 1, 5 ==> 4
        // 2, 1 ==> 5

        let map = Map {
                    number_of_rows: 5,
                    number_of_cols: 5,
                    trees: vec![
                        3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0
                    ]
                };

        assert_eq!(map.row_col_to_index(1, 1), 0);
        assert_eq!(map.row_col_to_index(1, 2), 1);
        assert_eq!(map.row_col_to_index(1, 3), 2);
        assert_eq!(map.row_col_to_index(1, 4), 3);
        assert_eq!(map.row_col_to_index(1, 5), 4);
        assert_eq!(map.row_col_to_index(2, 1), 5);
        
    }

    #[test]
    fn test_input_example() {
        assert_eq!(input_generator(
"30373
25512
65332
33549
35390"
        ),
        Some(
                Map {
                    number_of_rows: 5,
                    number_of_cols: 5,
                    trees: vec![
                        3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0
                    ]
                }
            )
        )
    }
        #[test]
    fn test_input_example_outer_rim() {
        assert_eq!(input_generator(
"30373
25512
65332
33549
35390"
        ).unwrap().count_outer_rim(),
        Ok(16)
    )
    }
    
    #[test]
    fn test_part1() {

        assert_eq!(part1(
                &Map {
                    number_of_rows: 5,
                    number_of_cols: 5,
                    trees: vec![
                        3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0
                    ]
                }
            ),
            Some(21)
        )
    }


    #[test]
    fn test_part2() {
        assert_eq!(part2(
                &Map {
                    number_of_rows: 5,
                    number_of_cols: 5,
                    trees: vec![
                        3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0
                    ]
                }
            ),
            Some(8)
        )
    }

    #[test]
    fn test_part1_bigger_set() {

/*
10002111211020
11111020133232 //9
22210012330023 //5
21221031131010 //5
11200210232200
*/
        assert_eq!(part1(
&input_generator(
"10002111211020
11111020133232
22210012330023
21221031131010
11200210232200"
).unwrap()
            ),
            Some(53)
        )
    }
}
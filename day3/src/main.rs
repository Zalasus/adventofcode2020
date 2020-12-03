
use std::convert::TryInto;
use std::ops::Index;

struct Map {
    width: usize,
    height: usize,
    points: Vec<bool>
}

impl Map {

    pub fn new_from_str(map: &str) -> Self {
        let mut width : Option<usize> = None;
        let mut height = 0;
        let mut points : Vec<bool> = Vec::new();
        for line in map.split('\n').filter(|l| l.len() > 0) {
            match width {
                Some(w) => assert_eq!(w, line.len()),
                None => width = Some(line.len())
            }
            points.extend(line.chars().map(|c| c == '#'));
            height += 1;
        }

        Self {
            width: width.unwrap(),
            height,
            points
        }
    }
}

fn modulus(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

impl Index<(isize,isize)> for Map {

    type Output = bool;

    fn index(&self, index: (isize, isize)) -> &bool {
        let w : isize = self.width.try_into().unwrap();
        let h : isize = self.height.try_into().unwrap();

        if (index.1 < 0) || (index.1 >= h) {
            panic!("Map Y index out of bounds. Requested y={}, but height={}", index.1, h);
        }
        let offset = modulus(index.0,w) + index.1*w;
        let offset_u : usize = offset.try_into().unwrap();
        &self.points[offset_u]
    }
}

fn count_trees(map: &Map, slope_x: isize, slope_y: isize) -> usize {
    let height : isize = map.height.try_into().unwrap();
    let mut x : isize = 0;
    let mut y : isize = 0;
    let mut trees_encountered = 0;
    while (y >= 0) && (y < height) {
        let is_tree = map[(x,y)];
        println!("Walking to {}/{}. Tree: {}", x, y, is_tree);
        if is_tree {
            trees_encountered += 1;
        }
        x += slope_x;
        y += slope_y;
    }
    trees_encountered
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAP : &str = concat!(  "..##.......\n",
                                      "#...#...#..\n",
                                      ".#....#..#.\n",
                                      "..#.#...#.#\n",
                                      ".#...##..#.\n",
                                      "..#.##.....\n",
                                      ".#.#.#....#\n",
                                      ".#........#\n",
                                      "#.##...#...\n",
                                      "#...##....#\n",
                                      ".#..#...#.#\n");

    #[test]
    fn map_access() {
        let map = Map::new_from_str(TEST_MAP);

        assert_eq!(map[(0,0)], false);
        assert_eq!(map[(2,0)], true);
        assert_eq!(map[(0,1)], true);
        assert_eq!(map[(4,3)], true);
        assert_eq!(map[(4,4)], false);

        // wrapping
        assert_eq!(map[(-2,3)], false);
        assert_eq!(map[(-3,3)], true);
        assert_eq!(map[(11,2)], false);
        assert_eq!(map[(12,2)], true);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let map = Map::new_from_str(TEST_MAP);
        map[(3,11)];
    }

    #[test]
    fn tree_count() {
        let map = Map::new_from_str(TEST_MAP);
        assert_eq!(count_trees(&map, 3, 1), 7);
    }

    #[test]
    fn multi_slope() {
        let map = Map::new_from_str(TEST_MAP);

        let trees1 = count_trees(&map, 1, 1);
        assert_eq!(trees1, 2);

        let trees2 = count_trees(&map, 3, 1);
        assert_eq!(trees2, 7);

        let trees3 = count_trees(&map, 5, 1);
        assert_eq!(trees3, 3);

        let trees4 = count_trees(&map, 7, 1);
        assert_eq!(trees4, 4);

        let trees5 = count_trees(&map, 1, 2);
        assert_eq!(trees5, 2);

        assert_eq!(trees1*trees2*trees3*trees4*trees5, 336);
    }
}


fn main() {
    let map_string = std::fs::read_to_string("day3_input.txt").unwrap();
    let map = Map::new_from_str(&map_string);

    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut mult_trees = 1;
    for slope in &slopes {
        let trees = count_trees(&map, slope.0, slope.1);
        println!("Encountered {} trees with slope {}/{}", trees, slope.0, slope.1);

        mult_trees *= trees;
    }

    println!("Multiplied together: {}", mult_trees);
}


extern crate bit_vec;

type Vector3 = (isize, isize, isize);

#[derive(Copy)]
struct Cube {
    start: Vector3,
    dims: Vector3
}

impl Cube {
    pub fn iter(&self) -> CubeIter {
        CubeIter {
            self,
            point: (0, 0, 0)
        }
    }
}

struct CubeIter<'a> {
    cube: &'a Cube,
    point: Vector3
}

impl<'a> Iterator for CubeIter<'a> {

    type Item = Vector3;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.point + self.cube.start;
        self.point.0 += 1;
        if self.point.0 >= self.cube.dims.0 {
            self.point.1 += 1;
            self.point.0 = 0;
            if self.point.1 >= self.cube.dims.1 {
                self.point.2 += 1;
                self.point.1 = 0;
                if self.point.2 >= self.cube.dims.2 {
                    return None;
                }
            }
        }
        Some(result)
    }
}

struct Space {
    extends: Cube,
    state: BitVec,
    next_state: BitVec
}

const NEIGHBOURHOOD : Cube = Cube::new((-1, -1, -1), (1, 1, 1));

impl Space {

    pub fn new() -> Self {
        Self{
            extends: Cube::new((0,0,0), (0,0,0)),
            state: BitVec::new(),
            next_state: BitVec::new()
        }
    }

    fn to_index(&self, p: Vector3) -> usize {
        let b = p - self.extends.start;
        b.0 + b.1*self.extends.dims.0 + b.2*self.extends.dims.0*self.extends.dims.1
    }

    pub fn step(&mut self) {

        // first, step the active area
        for p in self.extends.iter() {
            let this_index = self.to_index(p);

            let mut ones = 0;
            for offset in NEIGHBOURHOOD.iter() {
                ones += self.state[self.to_index(p + offset)] as usize;
            }

            let next = if self.state[this_index] {
                // ones also counts center, so amounts are increased
                ones == 3 || ones == 4
            }else{
                ones == 3
            }

            self.next_state[this_index] = next;
        }

        // then, iterate over the area that's one step outside of the active area. if any cells
        //  become active there, grow the space in the respective direction
        for p in self.extends.top().iter() {
            
        }
    }

    pub fn grow(&mut self) {
        let mut new_extends = self.extends;
        new_extends.start -= (1, 1, 1);
        new_extends.dims += (1, 1, 1);

        let bit_count = new_extends.dims.0 * new_extends.dims.1 * new_extends.dims.2;
        self.next_state.reserve(bit_count);

    }

}

fn main() {
    println!("Hello, world!");
}

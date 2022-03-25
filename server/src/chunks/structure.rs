use super::blocks::Block;

#[derive(Clone, Debug)]
pub struct Structure {
    pub size: [usize; 3],
    pub voxels: Vec<Vec<Vec<Block>>>,
}
impl Structure {
    pub fn new(size: [usize; 3]) -> Self {
        Self {
            size,
            voxels: vec![vec![vec![Block::None; size[2]]; size[1]]; size[0]],
        }
    }

    pub fn place(&mut self, coord: [usize; 3], block: Block) -> bool {
        if coord[0] < self.size[0] && coord[1] < self.size[1] && coord[2] < self.size[2] {
            self.voxels[coord[0]][coord[1]][coord[2]] = block;
        } else {
            return false;
        }
        return true;
    }

    pub fn get(&self, coord: [usize; 3]) -> Option<Block> {
        if coord[0] < self.size[0] && coord[1] < self.size[1] && coord[2] < self.size[2] {
            return Some(self.voxels[coord[0]][coord[1]][coord[2]]);
        } else {
            return None;
        }
    }
}

pub fn generate_tree() -> Structure {
    let mut tree = Structure::new([20, 25, 20]);

    let rad: i32 = 9;
    for x in -rad..rad {
        for y in -rad..rad {
            for z in -rad..rad {
                if x.pow(2) + z.pow(2) + y.pow(2) >= rad.pow(2) {
                    continue;
                }
                tree.place([x as usize + 10, y as usize + 15, z as usize + 10], Block::Leave);
            }
        }
    }

    for y in 0..23 {
        tree.place([10, y, 10], Block::Dirt);
    }

    tree
}

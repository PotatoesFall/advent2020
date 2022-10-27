use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug)]
pub struct State3 {
    active: HashSet<Point3>,
}

impl State3 {
    pub fn new() -> State3 {
        State3 {
            active: HashSet::new(),
        }
    }

    pub fn set_active(&mut self, v: Point3) {
        self.active.insert(v);
    }

    pub fn set_inactive(&mut self, v: &Point3) {
        self.active.remove(v);
    }

    pub fn is_active(&self, p: &Point3) -> bool {
        self.active.contains(p)
    }

    pub fn range(&self) -> Cuboid3 {
        let first = self.active.iter().next().unwrap();
        let mut range = Cuboid3 {
            min_x: first.x,
            min_y: first.y,
            min_z: first.z,
            max_x: first.x,
            max_y: first.y,
            max_z: first.z,
        };

        for point in self.active.iter() {
            range.min_x = range.min_x.min(point.x);
            range.min_y = range.min_y.min(point.y);
            range.min_z = range.min_z.min(point.z);
            range.max_x = range.max_x.max(point.x);
            range.max_y = range.max_y.max(point.y);
            range.max_z = range.max_z.max(point.z);
        }

        range
    }

    pub fn active_count(&self) -> i64 {
        self.active.len() as i64
    }
}

impl std::fmt::Display for State3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let range = self.range();
        // println!("{:?}", range);
        for z in range.min_z..range.max_z + 1 {
            write!(f, "z={}\n", z)?;
            for y in range.min_y..range.max_y + 1 {
                for x in range.min_x..range.max_x + 1 {
                    if self.is_active(&Point3 { x, y, z }) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }

                write!(f, "\n")?;
            }

            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

#[derive(Debug)]
pub struct Cuboid3 {
    pub min_x: i64,
    pub min_y: i64,
    pub min_z: i64,
    pub max_x: i64,
    pub max_y: i64,
    pub max_z: i64,
}

impl Cuboid3 {
    pub fn iter(&self) -> CuboidIter3<'_> {
        CuboidIter3 {
            cuboid: self,
            cursor: Point3 {
                x: self.min_x,
                y: self.min_y,
                z: self.min_z,
            },
        }
    }

    pub fn expand(&self, n: i64) -> Cuboid3 {
        Cuboid3 {
            min_x: self.min_x - n,
            min_y: self.min_y - n,
            min_z: self.min_z - n,
            max_x: self.max_x + n,
            max_y: self.max_y + n,
            max_z: self.max_z + n,
        }
    }
}

pub struct CuboidIter3<'a> {
    cuboid: &'a Cuboid3,
    cursor: Point3,
}

impl<'a> Iterator for CuboidIter3<'a> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.z > self.cuboid.max_z {
            return None;
        }

        let return_value = self.cursor;

        self.cursor.x += 1;

        if self.cursor.x == self.cuboid.max_x + 1 {
            self.cursor.x = self.cuboid.min_x;
            self.cursor.y += 1;

            if self.cursor.y == self.cuboid.max_y + 1 {
                self.cursor.y = self.cuboid.min_y;
                self.cursor.z += 1;

                if self.cursor.z == self.cuboid.max_z + 1 {
                    self.cursor.z += 1;
                }
            }
        }

        Some(return_value)
    }
}

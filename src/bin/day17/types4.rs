use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

#[derive(Debug)]
pub struct State4 {
    active: HashSet<Point4>,
}

impl State4 {
    pub fn new() -> State4 {
        State4 {
            active: HashSet::new(),
        }
    }

    pub fn set_active(&mut self, v: Point4) {
        self.active.insert(v);
    }

    pub fn set_inactive(&mut self, v: &Point4) {
        self.active.remove(v);
    }

    pub fn is_active(&self, p: &Point4) -> bool {
        self.active.contains(p)
    }

    pub fn range(&self) -> Cuboid4 {
        let first = self.active.iter().next().unwrap();
        let mut range = Cuboid4 {
            min_x: first.x,
            min_y: first.y,
            min_z: first.z,
            min_w: first.w,
            max_x: first.x,
            max_y: first.y,
            max_z: first.z,
            max_w: first.w,
        };

        for point in self.active.iter() {
            range.min_x = range.min_x.min(point.x);
            range.min_y = range.min_y.min(point.y);
            range.min_z = range.min_z.min(point.z);
            range.min_w = range.min_w.min(point.w);
            range.max_x = range.max_x.max(point.x);
            range.max_y = range.max_y.max(point.y);
            range.max_z = range.max_z.max(point.z);
            range.max_w = range.max_w.max(point.w);
        }

        range
    }

    pub fn active_count(&self) -> i64 {
        self.active.len() as i64
    }
}

impl std::fmt::Display for State4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let range = self.range();
        // println!("{:?}", range);
        for w in range.min_w..range.max_w + 1 {
            for z in range.min_z..range.max_z + 1 {
                write!(f, "z={}, w={}\n", z, w)?;
                for y in range.min_y..range.max_y + 1 {
                    for x in range.min_x..range.max_x + 1 {
                        if self.is_active(&Point4 { x, y, z, w }) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }

                    write!(f, "\n")?;
                }

                write!(f, "\n")?;
            }
        }

        write!(f, "\n")
    }
}

#[derive(Debug, PartialEq)]
pub struct Cuboid4 {
    pub min_x: i64,
    pub min_y: i64,
    pub min_z: i64,
    pub min_w: i64,
    pub max_x: i64,
    pub max_y: i64,
    pub max_z: i64,
    pub max_w: i64,
}

impl Cuboid4 {
    pub fn iter(&self) -> CuboidIter4<'_> {
        CuboidIter4 {
            cuboid: self,
            cursor: Point4 {
                x: self.min_x,
                y: self.min_y,
                z: self.min_z,
                w: self.min_w,
            },
        }
    }

    pub fn expand(&self, n: i64) -> Cuboid4 {
        Cuboid4 {
            min_x: self.min_x - n,
            min_y: self.min_y - n,
            min_z: self.min_z - n,
            min_w: self.min_w - n,
            max_x: self.max_x + n,
            max_y: self.max_y + n,
            max_z: self.max_z + n,
            max_w: self.max_w + n,
        }
    }
}

pub struct CuboidIter4<'a> {
    cuboid: &'a Cuboid4,
    cursor: Point4,
}

impl<'a> Iterator for CuboidIter4<'a> {
    type Item = Point4;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.w > self.cuboid.max_w {
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
                    self.cursor.z = self.cuboid.min_z;
                    self.cursor.w += 1;
                }
            }
        }

        Some(return_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> State4 {
        State4 {
            active: HashSet::from([
                Point4 {
                    x: -1,
                    y: -2,
                    z: -3,
                    w: -4,
                },
                Point4 {
                    x: 1,
                    y: 2,
                    z: 3,
                    w: 4,
                },
            ]),
        }
    }

    #[test]
    fn state_range() {
        let state = test_state();
        let expected = Cuboid4 {
            min_x: -1,
            max_x: 1,
            min_y: -2,
            max_y: 2,
            min_z: -3,
            max_z: 3,
            min_w: -4,
            max_w: 4,
        };

        assert_eq!(state.range(), expected);
    }

    #[test]
    fn state_active_count() {
        let state = test_state();
        assert_eq!(state.active_count(), 2);
    }

    #[test]
    fn state_is_active() {
        let state = test_state();
        let mut p = Point4 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
        };

        assert!(state.is_active(&p));

        p.x = 100;
        assert!(!state.is_active(&p));
    }

    #[test]
    fn state_set_active_inactive() {
        let mut state = test_state();
        let p1 = Point4 {
            x: 1,
            y: 1,
            z: 1,
            w: 1,
        };
        let p2 = Point4 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
        };

        state.set_active(p1);
        state.set_inactive(&p2);

        assert_eq!(state.active_count(), 2);
        assert!(state.is_active(&p1));
        assert!(!state.is_active(&p2));
    }

    #[test]
    fn cuboid_iter() {
        let cuboid = Cuboid4 {
            min_x: -1,
            max_x: 1,
            min_y: -1,
            max_y: 1,
            min_z: -1,
            max_z: 1,
            min_w: -1,
            max_w: 1,
        };

        let all_points: Vec<Point4> = cuboid.iter().collect();

        assert_eq!(all_points.len(), 81);

        assert!(all_points.contains(&Point4 {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }));

        assert!(all_points.contains(&Point4 {
            x: 1,
            y: -1,
            z: -1,
            w: 1,
        }));

        assert!(all_points.contains(&Point4 {
            x: -1,
            y: 1,
            z: 1,
            w: -1,
        }));
    }
}

use std::ops::RangeInclusive;

fn main() {
    // TODO: parsing
    // target area: x=195..238, y=-93..-67

    let target = Target {
        x: 195..=238,
        y: -93..=-67,
    };

    println!("Part 1: {}", part1(&target));
    println!("Part 2: {}", part2(&target));
}

#[derive(Debug)]
struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Default, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

fn part1(target: &Target) -> i64 {
    let vels = hitting_velocities(target);

    let vel = vels.iter().max_by_key(|vel| vel.y).unwrap();
    let mut max_y = 0;

    target.run_while(*vel, |pos, _| {
        if pos.y < max_y {
            return false;
        }
        max_y = pos.y;
        true
    });

    max_y
}

fn part2(target: &Target) -> i64 {
    hitting_velocities(target).len() as _
}

fn hitting_velocities(target: &Target) -> Vec<Velocity> {
    let mut vels = vec![];

    for x in 0..=*target.x.end() {
        for y in *target.y.start()..=*target.x.end() {
            let vel = Velocity { x, y };

            if target.hits(vel) {
                vels.push(vel);
            }
        }
    }

    vels
}

impl Target {
    fn hits(&self, velocity: Velocity) -> bool {
        let pos = self.run_while(velocity, |pos, vel| {
            vel.y > 0 || pos.y >= *self.y.start()
        });
        pos.inside(self)
    }

    fn run_while<F>(&self, mut velocity: Velocity, mut test: F) -> Pos
    where
        F: FnMut(&Pos, &Velocity) -> bool,
    {
        let mut pos = Pos::default();

        while test(&pos, &velocity) {
            pos.inc(&velocity);

            velocity.apply_drag();
            velocity.apply_gravity();

            if pos.inside(self) {
                break;
            }
        }

        pos
    }
}

impl Velocity {
    fn apply_drag(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.x < 0 {
            self.x += 1;
        }
    }

    fn apply_gravity(&mut self) {
        self.y -= 1;
    }
}

impl Pos {
    fn inc(&mut self, vel: &Velocity) {
        self.x += vel.x;
        self.y += vel.y;
    }

    fn inside(&self, target: &Target) -> bool {
        target.x.contains(&self.x) && target.y.contains(&self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TARGET: Target = Target {
        x: 20..=30,
        y: -10..=-5,
    };

    #[test]
    fn test_velocity() {
        assert!(TARGET.hits(Velocity { x: 7, y: 2 }));
        assert!(TARGET.hits(Velocity { x: 6, y: 3 }));
        assert!(TARGET.hits(Velocity { x: 9, y: 0 }));
        assert!(!TARGET.hits(Velocity { x: 17, y: -4 }));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TARGET), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TARGET), 112);
    }
}

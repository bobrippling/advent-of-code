use std::ops::*;

type Val = isize;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Vec3 {
	x: Val,
	y: Val,
	z: Val,
}

#[allow(dead_code)]
fn unit(v: Val) -> Val {
	if v > 0 {
		1
	} else if v < 0 {
		-1
	} else {
		0
	}
}

fn to(v: Val, u: Val) -> Val {
	if v > u {
		-1
	} else if v < u {
		1
	} else {
		0
	}
}

impl Vec3 {
	fn zero() -> Self {
		Vec3 { x: 0, y: 0, z: 0 }
	}

	fn new(x: Val, y: Val, z: Val) -> Self {
		Vec3 { x, y, z }
	}

	#[allow(dead_code)]
	fn unit(self) -> Self {
		Vec3 {
			x: unit(self.x),
			y: unit(self.y),
			z: unit(self.z),
		}
	}

	fn to(self, rhs: Self) -> Self {
		Vec3 {
			x: to(self.x, rhs.x),
			y: to(self.y, rhs.y),
			z: to(self.z, rhs.z),
		}
	}

	fn abssum(&self) -> Val {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl AddAssign<Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		*self = *self + rhs;
	}
}

/*
impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}
*/

impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3 {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

#[derive(PartialEq, Eq, Debug)]
struct Pos(Vec3);

#[derive(PartialEq, Eq, Debug)]
struct Velocity(Vec3);

#[derive(PartialEq, Eq, Debug)]
struct Moon {
	pos: Pos,
	vel: Velocity,
}

impl Moon {
	fn new(x: Val, y: Val, z: Val) -> Self {
		Moon {
			pos: Pos(Vec3 { x, y, z }),
			vel: Velocity(Vec3::zero()),
		}
	}

	fn potential_energy(&self) -> Val {
		self.pos.0.abssum()
	}

	fn kinetic_energy(&self) -> Val {
		self.vel.0.abssum()
	}
}

struct System {
	moons: Vec<Moon>,
}

#[derive(PartialEq, Debug)]
struct Change {
	i: usize,
	from: usize,
	v: Vec3,
}

fn changes_for_moons(
	(m_i, i): (&Moon, usize),
	(m_j, j): (&Moon, usize),
) -> Vec<Change> {
	let diff = m_j.pos.0.to(m_i.pos.0); //.unit();
	let mut changes = Vec::new();

	changes.push(Change {
		i,
		from: j,
		v: -diff,
	});
	/*changes.push(Change {
		i: j,
		from: i,
		v: diff,
	});*/

	changes
}

fn apply_changes(moons: &mut Vec<Moon>, changes: &Vec<Change>) {
	for Change { i, from: _, v } in changes {
		/*if *i == 0 {
			println!("moons[{}].vel += {:?} (from {})", *i, *v, *from);
		}*/

		moons[*i].vel.0 += *v;
	}

	//for moon in moons {
}

impl System {
	fn step(&mut self) {
		self.apply_gravity();
		self.apply_velocity();
		//self.time += 1;
	}

	fn apply_gravity(&mut self) {
		let mut changes = Vec::new();

		for (moon_i, i) in (&self.moons).iter().zip(0..) {
			for (moon_j, j) in (&self.moons).iter().zip(0..) {
				if i == j {
					continue;
				}

				//println!("{} influencing {}", i, j);

				changes.extend(
					changes_for_moons(
						(moon_i, i),
						(moon_j, j)));
			}
		}

		apply_changes(&mut self.moons, &changes);
	}

	fn apply_velocity(&mut self) {
		for moon in self.moons.iter_mut() {
			moon.pos.0 += moon.vel.0;
		}
	}

	fn energy(&self) -> Val {
		self.moons
			.iter()
			.map(|m| m.kinetic_energy() * m.potential_energy())
			.sum()
	}
}

fn main() {
	let mut sys = System {
		moons: vec![
			Moon::new(-9, -1, -1),
			Moon::new(2, 9, 5),
			Moon::new(10, 18, -12),
			Moon::new(-6, 15, -7),
		],
	};

	for _ in 1..=1000 {
		sys.step();
	}

	println!("{}", sys.energy());
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day12_diff() {
		let callisto = Vec3::new(5, 0, 0); // should be -1 to vel
		let ganymede = Vec3::new(3, 0, 0); // should be +1 to vel
		let ganymede2 = Vec3::new(3, 5, 0); // should be +1 to vel

		assert_eq!(callisto.to(ganymede), Vec3::new(-1, 0, 0));
		assert_eq!(ganymede.to(callisto), Vec3::new( 1, 0, 0));
		assert_eq!(ganymede.to(ganymede2), Vec3::new(0, 1, 0));

		//let diff = callisto - ganymede;
		//assert_eq!(diff, Vec3::new(2, 0, 0));
		//assert_eq!(callisto + diff.unit(), Vec3::new(6, 0, 0));
	}

	/*#[test]
	fn test_day12_bit_by_bit() {
		let callisto = Moon { pos: Pos(Vec3::new(5, 7, 0)), vel: Velocity(Vec3::zero()) };
		let ganymede = Moon { pos: Pos(Vec3::new(3, 7, 0)), vel: Velocity(Vec3::zero()) };

		let changes = changes_for_moons((&callisto, 0), (&ganymede, 1));
		assert_eq!(
			changes,
			vec![
				Change { i: 0, v: Vec3 { x: -1, y: 0, z: 0 }, from: 1 },
				Change { i: 1, v: Vec3 { x:  1, y: 0, z: 0 }, from: 0 },
			]);

		let mut moons = vec![callisto, ganymede];
		apply_changes(&mut moons, &changes);

		assert_eq!(
			moons,
			vec![
				Moon { pos: Pos(Vec3::new(5, 7, 0)), vel: Velocity(Vec3 { x: -1, y: 0, z: 0 }) },
				Moon { pos: Pos(Vec3::new(3, 7, 0)), vel: Velocity(Vec3 { x:  1, y: 0, z: 0 }) },
			]);

		let mut sys = System { moons };
		sys.apply_velocity();

		assert_eq!(
			sys.moons,
			vec![
				Moon { pos: Pos(Vec3::new(4, 7, 0)), vel: Velocity(Vec3 { x: -1, y: 0, z: 0 }) },
				Moon { pos: Pos(Vec3::new(4, 7, 0)), vel: Velocity(Vec3 { x:  1, y: 0, z: 0 }) },
			]);
	}
	*/

	#[test]
	fn test_day12_velocity() {
		let mut europa = Moon {
			pos: Pos(Vec3 { x: 1, y: 2, z: 3 }),
			vel: Velocity(Vec3 { x: -2, y: 0, z: 3 }),
		};

		europa.pos.0 += europa.vel.0;

		assert_eq!(europa.pos, Pos(Vec3 {x:-1, y:2, z:6 }));
	}

	#[test]
	fn test_day12_eg1() {
		let mut sys = System {
			moons: vec![
				Moon::new(-1, 0, 2),
				Moon::new(2, -10, -7),
				Moon::new(4, -8, 8),
				Moon::new(3, 5, -1),
			],
		};

		assert_eq!(
			sys.moons,
			vec![
				Moon { pos: Pos(Vec3::new(-1,   0,  2)), vel: Velocity(Vec3::new( 0,  0,  0)) },
				Moon { pos: Pos(Vec3::new( 2, -10, -7)), vel: Velocity(Vec3::new( 0,  0,  0)) },
				Moon { pos: Pos(Vec3::new( 4,  -8,  8)), vel: Velocity(Vec3::new( 0,  0,  0)) },
				Moon { pos: Pos(Vec3::new( 3,   5, -1)), vel: Velocity(Vec3::new( 0,  0,  0)) },
			]);

		sys.step();
		// [0].x: +1 from [1], +1 from [2], +1 from [3] -->  3 (x vel)
		// [0].y: -1 from [1], -1 from [2], +1 from [3] --> -1 (y vel)
		// [0].z: -1 from [1], +1 from [2], -1 from [3] --> -1 (z vel)
		assert_eq!(
			sys.moons,
			vec![
				Moon { pos: Pos(Vec3::new( 2, -1,  1)), vel: Velocity(Vec3::new( 3, -1, -1)) },
				Moon { pos: Pos(Vec3::new( 3, -7, -4)), vel: Velocity(Vec3::new( 1,  3,  3)) },
				Moon { pos: Pos(Vec3::new( 1, -7,  5)), vel: Velocity(Vec3::new(-3,  1, -3)) },
				Moon { pos: Pos(Vec3::new( 2,  2,  0)), vel: Velocity(Vec3::new(-1, -3,  1)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 5, -3, -1)), vel: Velocity(Vec3::new( 3, -2, -2)) },
			Moon { pos: Pos(Vec3::new( 1, -2,  2)), vel: Velocity(Vec3::new(-2,  5,  6)) },
			Moon { pos: Pos(Vec3::new( 1, -4, -1)), vel: Velocity(Vec3::new( 0,  3, -6)) },
			Moon { pos: Pos(Vec3::new( 1, -4,  2)), vel: Velocity(Vec3::new(-1, -6,  2)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 5, -6, -1)), vel: Velocity(Vec3::new( 0, -3,  0)) },
			Moon { pos: Pos(Vec3::new( 0,  0,  6)), vel: Velocity(Vec3::new(-1,  2,  4)) },
			Moon { pos: Pos(Vec3::new( 2,  1, -5)), vel: Velocity(Vec3::new( 1,  5, -4)) },
			Moon { pos: Pos(Vec3::new( 1, -8,  2)), vel: Velocity(Vec3::new( 0, -4,  0)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 2, -8,  0)), vel: Velocity(Vec3::new(-3, -2,  1)) },
			Moon { pos: Pos(Vec3::new( 2,  1,  7)), vel: Velocity(Vec3::new( 2,  1,  1)) },
			Moon { pos: Pos(Vec3::new( 2,  3, -6)), vel: Velocity(Vec3::new( 0,  2, -1)) },
			Moon { pos: Pos(Vec3::new( 2, -9,  1)), vel: Velocity(Vec3::new( 1, -1, -1)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new(-1, -9,  2)), vel: Velocity(Vec3::new(-3, -1,  2)) },
			Moon { pos: Pos(Vec3::new( 4,  1,  5)), vel: Velocity(Vec3::new( 2,  0, -2)) },
			Moon { pos: Pos(Vec3::new( 2,  2, -4)), vel: Velocity(Vec3::new( 0, -1,  2)) },
			Moon { pos: Pos(Vec3::new( 3, -7, -1)), vel: Velocity(Vec3::new( 1,  2, -2)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new(-1, -7,  3)), vel: Velocity(Vec3::new( 0,  2,  1)) },
			Moon { pos: Pos(Vec3::new( 3,  0,  0)), vel: Velocity(Vec3::new(-1, -1, -5)) },
			Moon { pos: Pos(Vec3::new( 3, -2,  1)), vel: Velocity(Vec3::new( 1, -4,  5)) },
			Moon { pos: Pos(Vec3::new( 3, -4, -2)), vel: Velocity(Vec3::new( 0,  3, -1)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 2, -2,  1)), vel: Velocity(Vec3::new( 3,  5, -2)) },
			Moon { pos: Pos(Vec3::new( 1, -4, -4)), vel: Velocity(Vec3::new(-2, -4, -4)) },
			Moon { pos: Pos(Vec3::new( 3, -7,  5)), vel: Velocity(Vec3::new( 0, -5,  4)) },
			Moon { pos: Pos(Vec3::new( 2,  0,  0)), vel: Velocity(Vec3::new(-1,  4,  2)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 5,  2, -2)), vel: Velocity(Vec3::new( 3,  4, -3)) },
			Moon { pos: Pos(Vec3::new( 2, -7, -5)), vel: Velocity(Vec3::new( 1, -3, -1)) },
			Moon { pos: Pos(Vec3::new( 0, -9,  6)), vel: Velocity(Vec3::new(-3, -2,  1)) },
			Moon { pos: Pos(Vec3::new( 1,  1,  3)), vel: Velocity(Vec3::new(-1,  1,  3)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 5,  3, -4)), vel: Velocity(Vec3::new( 0,  1, -2)) },
			Moon { pos: Pos(Vec3::new( 2, -9, -3)), vel: Velocity(Vec3::new( 0, -2,  2)) },
			Moon { pos: Pos(Vec3::new( 0, -8,  4)), vel: Velocity(Vec3::new( 0,  1, -2)) },
			Moon { pos: Pos(Vec3::new( 1,  1,  5)), vel: Velocity(Vec3::new( 0,  0,  2)) },
			]);

		sys.step();
		assert_eq!(
			sys.moons,
			vec![
			Moon { pos: Pos(Vec3::new( 2,  1, -3)), vel: Velocity(Vec3::new(-3, -2,  1)) },
			Moon { pos: Pos(Vec3::new( 1, -8,  0)), vel: Velocity(Vec3::new(-1,  1,  3)) },
			Moon { pos: Pos(Vec3::new( 3, -6,  1)), vel: Velocity(Vec3::new( 3,  2, -3)) },
			Moon { pos: Pos(Vec3::new( 2,  0,  4)), vel: Velocity(Vec3::new( 1, -1, -1)) },
			]);

		assert_eq!(sys.energy(), 179);
	}
}

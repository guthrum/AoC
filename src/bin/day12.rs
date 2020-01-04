use std::cmp::Ordering;
use std::fmt;

fn gcm(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcm(b, a % b),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a.abs() * b.abs()) / gcm(a, b)
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Coordinate { x, y, z }
    }

    fn apply_velocity(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
        self.z += velocity.z;
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Debug)]
struct Moon {
    position: Coordinate,
    velocity: Velocity,
}

impl Moon {
    fn new(position: Coordinate) -> Self {
        Moon {
            position,
            velocity: Velocity { x: 0, y: 0, z: 0 },
        }
    }

    #[allow(dead_code)]
    fn get_kinetic_energy(&self) -> i64 {
        let velocity = &self.velocity;
        velocity.x.abs() + velocity.y.abs() + velocity.z.abs()
    }

    #[allow(dead_code)]
    fn get_potential_energy(&self) -> i64 {
        let pos = &self.position;
        pos.x.abs() + pos.y.abs() + pos.z.abs()
    }

    #[allow(dead_code)]
    fn get_total_energy(&self) -> i64 {
        self.get_kinetic_energy() * self.get_potential_energy()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        System { moons }
    }

    fn comp_velocity(vel1: i64, vel2: i64) -> i64 {
        match vel1.cmp(&vel2) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            _ => -1,
        }
    }

    fn simulate(&mut self, num_iterations: u64) {
        for _ in 0..num_iterations {
            {
                let comp_moons = self.moons.clone();
                for current_moon in self.moons.iter_mut() {
                    for comp_moon in comp_moons.iter() {
                        current_moon.velocity.x +=
                            Self::comp_velocity(current_moon.position.x, comp_moon.position.x);
                        current_moon.velocity.y +=
                            Self::comp_velocity(current_moon.position.y, comp_moon.position.y);
                        current_moon.velocity.z +=
                            Self::comp_velocity(current_moon.position.z, comp_moon.position.z);
                    }
                }
            }
            // apply velocity
            for current_moon in self.moons.iter_mut() {
                current_moon.position.apply_velocity(&current_moon.velocity);
            }
        }
    }

    #[allow(dead_code)]
    fn system_energy(&self) -> i64 {
        self.moons.iter().map(|x| x.get_total_energy()).sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for moon in &self.moons {
            println!("{}", moon);
        }
    }

    fn solve_part2(&mut self) -> i64 {
        let mut periods = (None, None, None);
        let start_x: Vec<(i64, i64)> = self
            .moons
            .iter()
            .map(|m| (m.position.x, m.velocity.x))
            .collect();
        let start_y: Vec<(i64, i64)> = self
            .moons
            .iter()
            .map(|m| (m.position.y, m.velocity.y))
            .collect();
        let start_z: Vec<(i64, i64)> = self
            .moons
            .iter()
            .map(|m| (m.position.z, m.velocity.z))
            .collect();
        let mut step = 0;

        while periods.0.is_none() || periods.1.is_none() || periods.2.is_none() {
            self.simulate(1);
            step += 1;
            if periods.0.is_none() {
                let x_state: Vec<(i64, i64)> = self
                    .moons
                    .iter()
                    .map(|m| (m.position.x, m.velocity.x))
                    .collect();
                if x_state == start_x {
                    periods.0 = Some(step);
                }
            }

            if periods.1.is_none() {
                let y_state: Vec<(i64, i64)> = self
                    .moons
                    .iter()
                    .map(|m| (m.position.y, m.velocity.y))
                    .collect();
                if y_state == start_y {
                    periods.1 = Some(step);
                }
            }

            if periods.2.is_none() {
                let z_state: Vec<(i64, i64)> = self
                    .moons
                    .iter()
                    .map(|m| (m.position.z, m.velocity.z))
                    .collect();
                if z_state == start_z {
                    periods.2 = Some(step);
                }
            }
        }
        lcm(
            periods.0.unwrap(),
            lcm(periods.1.unwrap(), periods.2.unwrap()),
        )
    }
}

fn main() {
    let moons = vec![
        Moon::new(Coordinate::new(3, 2, -6)),
        Moon::new(Coordinate::new(-13, 18, 10)),
        Moon::new(Coordinate::new(-8, -1, 13)),
        Moon::new(Coordinate::new(5, 10, 4)),
    ];
    let mut system = System::new(moons);
    println!("Part 2 = {}", system.solve_part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let moons = vec![
            Moon::new(Coordinate::new(3, 2, -6)),
            Moon::new(Coordinate::new(-13, 18, 10)),
            Moon::new(Coordinate::new(-8, -1, 13)),
            Moon::new(Coordinate::new(5, 10, 4)),
        ];
        let mut system = System::new(moons);
        system.simulate(1000);
        assert_eq!(system.system_energy(), 14780);
    }

    #[test]
    fn part2_example1() {
        let moons = vec![
            Moon::new(Coordinate::new(-1, 0, 2)),
            Moon::new(Coordinate::new(2, -10, -7)),
            Moon::new(Coordinate::new(4, -8, 8)),
            Moon::new(Coordinate::new(3, 5, -1)),
        ];
        let mut system = System::new(moons);
        assert_eq!(system.solve_part2(), 2772);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(1, 2), 2);
        assert_eq!(lcm(21, 6), 42);
    }
}

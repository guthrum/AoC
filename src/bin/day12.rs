use std::fmt;

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

    fn get_kinetic_energy(&self) -> i64 {
        let velocity = &self.velocity;
        velocity.x.abs() + velocity.y.abs() + velocity.z.abs()
    }

    fn get_potential_energy(&self) -> i64 {
        let pos = &self.position;
        pos.x.abs() + pos.y.abs() + pos.z.abs()
    }

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
    fn comp_velocity(vel1: i64, vel2: i64) -> i64 {
        if vel1 == vel2 {
            0
        } else if vel1 < vel2 {
            1
        } else {
            -1
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

    fn system_energy(&self) -> i64 {
        self.moons.iter().map(|x| x.get_total_energy()).sum()
    }

    fn print(&self) {
        for moon in &self.moons {
            println!("{}", moon);
        }
    }
}

fn main() {
    let moons = vec![
        Moon::new(Coordinate::new(3, 2, -6)),
        Moon::new(Coordinate::new(-13, 18, 10)),
        Moon::new(Coordinate::new(-8, -1, 13)),
        Moon::new(Coordinate::new(5, 10, 4)),
    ];
    let mut system = System { moons };
    system.simulate(1000);
    system.print();
    println!("system total energy = {}", system.system_energy());
}

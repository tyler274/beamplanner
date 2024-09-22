use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::time::Duration;

use crate::test_util::{check, fail, BOLD, CYAN, GRAY, GREEN, RED, RESET, YELLOW};
use crate::util::{Color, Sat, User, Vector3};

pub const TIMEOUT: Duration = Duration::from_secs(60);

pub struct Scenario {
    pub sats: HashMap<Sat, Vector3>,
    pub users: HashMap<User, Vector3>,
    pub min_coverage: f32,
}

impl Scenario {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            ..Default::default()
        };

        let file = std::fs::read_to_string(path)?;
        let lines = file.lines();
        for line in lines {
            let line = line
                .split('#')
                .next()
                .ok_or("Bad scenario string formatting")?
                .trim();

            if line.is_empty() {
                continue;
            }

            let mut parts = line.split_whitespace();
            let kind = parts.next().unwrap();
            match kind {
                "sat" => {
                    let id = parts.next().unwrap().parse()?;
                    let x = parts.next().unwrap().parse()?;
                    let y = parts.next().unwrap().parse()?;
                    let z = parts.next().unwrap().parse()?;
                    let pos = Vector3::new(x, y, z);
                    let sat = Sat::new(id);
                    s.sats.insert(sat, pos);
                }
                "user" => {
                    let id = parts.next().unwrap().parse()?;
                    let x = parts.next().unwrap().parse()?;
                    let y = parts.next().unwrap().parse()?;
                    let z = parts.next().unwrap().parse()?;
                    let pos = Vector3::new(x, y, z);
                    let user = User::new(id);
                    s.users.insert(user, pos);
                }
                "min_coverage" => {
                    s.min_coverage = parts.next().unwrap().parse()?;
                }
                _ => {
                    fail(&format!("Invalid token: {}", kind));
                }
            }
        }

        Ok(s)
    }

    pub fn check(&self, solution: &BTreeMap<User, (Sat, Color)>) {
        let mut beams: BTreeMap<Sat, BTreeSet<(Color, User)>> = BTreeMap::new();

        for (user, (sat, color)) in solution.iter() {
            let user_pos = self.users.get(user).unwrap();
            let sat_pos = self.sats.get(sat).unwrap();
            // Unnecessary due to the type system.
            check(
                *color == Color::A
                    || *color == Color::B
                    || *color == Color::C
                    || *color == Color::D,
                &format!("Invalid color: {}", color),
            );

            let angle = user_pos
                .unit()
                .dot((sat_pos - user_pos).unit())
                .acos()
                .to_degrees();

            check(
                angle <= 45.0,
                &format!(
                    "User {} cannot see satellite {} ({} degrees from vertical)",
                    user, sat, angle
                ),
            );

            beams
                .entry(*sat)
                .or_insert_with(BTreeSet::new)
                .insert((*color, *user));
        }

        for (sat, sat_beams) in beams.iter() {
            let sat_pos = self.sats.get(sat).unwrap();
            check(
                sat_beams.len() <= 32,
                &format!(
                    "Satellite {} cannot serve more than 32 users ({} assigned)",
                    sat,
                    sat_beams.len()
                ),
            );
            for (color_1, user_1) in sat_beams.iter() {
                for (color_2, user_2) in sat_beams.iter() {
                    if color_1 == color_2 && user_1 != user_1 {
                        let user_1_pos = self.users.get(user_1).unwrap();
                        let user_2_pos = self.users.get(user_2).unwrap();
                        let angle = sat_pos.angle_between(user_1_pos, user_2_pos);

                        check(
                            angle >= 10.0,
                            &format!(
                                "Users {} and {} on satellite {} {} are too close ({} degrees)",
                                user_1, user_2, sat, color_1, angle
                            ),
                        );
                    }
                }
            }

            let coverage = 1.0 * solution.len() as f32 / self.users.len() as f32;
            check(coverage >= self.min_coverage, "Too few users served")
        }
    }
}

impl Default for Scenario {
    fn default() -> Self {
        Self {
            sats: Default::default(),
            users: Default::default(),
            min_coverage: 1.0,
        }
    }
}

use crate::util::{Color, Sat, User, Vector3};
use std::collections::BTreeMap;

// In degrees
const MINIMUM_BEAM_ANGLE: f32 = 10.0;
const MAX_ALLOWABLE_BEAM_ANGLE: f32 = 45.0;
const MAX_ALLOWED_USERS: usize = 32;
const MAX_COLOR_OPTIONS: usize = 4;

type RowColIndex = (u64, u64);
type Vector = Vec<u64>;
// type ConnectionsMatrix = Vec<Vec<Color>>;
type ConnectionsMatrix = BTreeMap<Sat, BTreeMap<User, Color>>;
type Position = Vector3;

type Users = BTreeMap<User, Position>;
type Sats = BTreeMap<Sat, Position>;

type SatsSum = BTreeMap<Sat, u64>;
type UsersSum = BTreeMap<User, u64>;

type SortedUser = (User, f32, Position);
type SortedSat = (Sat, f32, Position);

type UsersSorted = Vec<SortedUser>;
type SatsSorted = Vec<SortedSat>;

enum UnsortedValues {
    Users(Vec<Position>),
    Sats(Vec<Position>),
}

enum SortedValues {
    Users(UsersSorted),
    Sats(SatsSorted),
}

// fn get_index_shifts(users: BTreeMap<User, Vector3>, sats: BTreeMap<Sat, Vector3>) -> RowColIndex {
//     let user_shift = users.len().trailing_zeros();
//     let sat_shift = sats.len().trailing_zeros();
//     (user_shift, sat_shift)
// }

fn sum_sats(valid_connections: ConnectionsMatrix) -> SatsSum {
    let mut sat_sums: BTreeMap<Sat, u64> = BTreeMap::new();
    for (sat, user_color) in valid_connections.iter() {
        for (user, color) in user_color.iter() {
            match valid_connections[sat][user] {
                Color::A | Color::B | Color::C | Color::D => {
                    sat_sums
                        .entry(*sat)
                        .and_modify(|curr| *curr += 1)
                        .or_insert(1);
                }
                Color::Init => (),
            }
        }
    }
    sat_sums
}

fn sum_users(valid_connections: ConnectionsMatrix) -> UsersSum {
    let mut users_sums: BTreeMap<User, u64> = BTreeMap::new();
    for user in valid_connections.values().next().unwrap().keys() {
        for (sat, _) in valid_connections.iter() {
            match valid_connections[sat][user] {
                Color::A | Color::B | Color::C | Color::D => {
                    users_sums
                        .entry(*user)
                        .and_modify(|curr| *curr += 1)
                        .or_insert(1);
                }
                Color::Init => (),
            }
        }
    }
    users_sums
}

fn initialize_valid_connections(num_sats: usize, num_users: usize) -> ConnectionsMatrix {
    // vec![vec![Color::Init; num_users]; num_sats]
    let valid_connections: ConnectionsMatrix = BTreeMap::new();
    valid_connections
}

fn beam_angle(user_pos: Vector3, sat_pos: Vector3) -> f32 {
    let center = Vector3::new(0.0, 0.0, 0.0);
    180.0 - user_pos.angle_between(&center, &sat_pos)
}

fn scaling_vector(x: f32, y: f32, z: f32, epsilon: f32) -> Vector3 {
    Vector3::new(x + epsilon, y + epsilon, z + epsilon)
}

// Normalize a vector of 3D points and sort by the dot product with the basis vector.
fn sort(values: UnsortedValues) -> SortedValues {
    // Add a small epsilon to avoid division by zero when normalizing
    let epsilon = 1e-10;
    let basis_vector = Vector3::basis();

    let lambda_max = |(max_x, max_y, max_z), v: &Vector3| {
        (
            f32::max(max_x, v.x),
            f32::max(max_y, v.y),
            f32::max(max_z, v.z),
        )
    };

    match values {
        UnsortedValues::Users(values) => {
            // Find the largest x, y, and z values for normalization
            let (largest_x, largest_y, largest_z) = values
                .iter()
                .fold((f32::MIN, f32::MIN, f32::MIN), lambda_max);

            let mut sorted_values: UsersSorted = Vec::new();

            for (i, value) in values.iter().enumerate() {
                let dot_product = value
                    .normalize_with(scaling_vector(largest_x, largest_y, largest_z, epsilon))
                    .dot(basis_vector);
                sorted_values.push((User(i as u64), dot_product, value.clone()));
            }

            // Sort by the dot product in descending order
            sorted_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            SortedValues::Users(sorted_values)
        }
        UnsortedValues::Sats(values) => {
            // Find the largest x, y, and z values for normalization
            let (largest_x, largest_y, largest_z) = values
                .iter()
                .fold((f32::MIN, f32::MIN, f32::MIN), lambda_max);

            let mut sorted_values: SatsSorted = Vec::new();

            for (i, value) in values.iter().enumerate() {
                let dot_product = value
                    .normalize_with(scaling_vector(largest_x, largest_y, largest_z, epsilon))
                    .dot(basis_vector);
                sorted_values.push((Sat(i as u64), dot_product, value.clone()));
            }

            // Sort by the dot product in descending order
            sorted_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            SortedValues::Sats(sorted_values)
        }
    }
}

fn get_sorted_values(users: Users, sats: Sats) -> (UsersSorted, SatsSorted) {
    // TODO: Rework the BTreeMap to sort the values directly?
    // let users_sorted: UsersSorted = sort(users.into_values().collect());
    // let sats_sorted: SatsSorted = sort(sats.into_values().collect());
    if let SortedValues::Users(users_sorted) =
        sort(UnsortedValues::Users(users.into_values().collect()))
    {
        if let SortedValues::Sats(sats_sorted) =
            sort(UnsortedValues::Sats(sats.into_values().collect()))
        {
            return (users_sorted, sats_sorted);
        } else {
            panic!("Failed to sort satellites");
        }
    } else {
        panic!("Failed to sort users");
    }
}

fn initialize_colors(
    users: UsersSorted,
    sats: SatsSorted,
    valid_connections: &mut ConnectionsMatrix,
) {
    let mut color = Color::A;
    for user_tuple in users {
        for sat_tuple in &sats {
            if valid_connections[&sat_tuple.0][&user_tuple.0] == Color::Init {
                valid_connections
                    .entry(sat_tuple.0)
                    .or_insert(Default::default())
                    .entry(user_tuple.0)
                    .and_modify(|curr| *curr = color);
                color = color.next();
            }
        }
    }
}

fn validate_satelite_congestion(
    users: UsersSorted,
    sats: SatsSorted,
    valid_connections: &mut ConnectionsMatrix,
    shift_colors: bool,
) {
    for sat_tuple in sats {
        // Construct a list with users that have valid connections to this satelite
        let users_with_valid_connections: Vec<(User, f32, Position)> = users
            .iter()
            .filter(|user_tuple| valid_connections[&sat_tuple.0][&user_tuple.0] != Color::Init)
            .map(|&user_tuple| user_tuple)
            .collect();

        // Check each user against each other user pairwise to check if they interfere
        for (i, user_tuple_1) in users_with_valid_connections.iter().enumerate() {
            for user_tuple_2 in users_with_valid_connections.iter().skip(i + 1) {
                // Find the angle between the satellite and the two users
                let angle = beam_angle(user_tuple_1.2, user_tuple_2.2);
                if angle < MINIMUM_BEAM_ANGLE {
                    // If the angle is too small, remove the connection with the lowest
                    // color or shift the colors if the flag is set
                    let color_1 = valid_connections[&sat_tuple.0][&user_tuple_1.0];
                    let color_2 = valid_connections[&sat_tuple.0][&user_tuple_2.0];
                    if shift_colors {
                        valid_connections.entry(sat_tuple.0).and_modify(|sat| {
                            sat.entry(user_tuple_2.0)
                                .and_modify(|color| *color = color_2.next());
                        });
                    } else {
                        if color_1 < color_2 {
                            valid_connections.entry(sat_tuple.0).and_modify(|sat| {
                                sat.entry(user_tuple_1.0)
                                    .and_modify(|color| *color = Color::Init);
                            });
                        } else {
                            valid_connections.entry(sat_tuple.0).and_modify(|sat| {
                                sat.entry(user_tuple_2.0)
                                    .and_modify(|color| *color = Color::Init);
                            });
                        }
                    }
                }
            }
        }
    }
}

fn remove_excess_satelites_per_user(
    valid_connections: ConnectionsMatrix,
    users: UsersSorted,
    sats: SatsSorted,
) {
    let sat_totals = sum_sats(valid_connections);

    // Remove extraneous satelites per user, if more than one satelite is assigned
    for user in users {
        // let user_sats = sat_totals[user.0];
    }
}

fn remove_excess_users_per_satellite(
    valid_connections: ConnectionsMatrix,
    users: UsersSorted,
    sats: SatsSorted,
) {
}

fn format_solution(
    valid_connections: ConnectionsMatrix,
    users: UsersSorted,
    sats: SatsSorted,
) -> BTreeMap<User, (Sat, Color)> {
    let solution: BTreeMap<User, (Sat, Color)> = BTreeMap::new();

    solution
}

/// Solves the satellite assignment problem.
///
/// Given a map of users and their positions, and a map of satellites and their positions,
/// this function assigns a satellite to each user and returns a map of users along with
/// their assigned satellite and color.
///
/// # Arguments
///
/// * `users` - A `BTreeMap` containing users and their positions.
/// * `sat` - A `BTreeMap` containing satellites and their positions.
///
/// # Returns
///
/// A `BTreeMap` containing users along with their assigned satellite and color.
pub fn solve(
    users: BTreeMap<User, Vector3>,
    sats: BTreeMap<Sat, Vector3>,
) -> BTreeMap<User, (Sat, Color)> {
    let mut solution = BTreeMap::new();

    let (users_sorted, sats_sorted) = get_sorted_values(users, sats);

    solution
}

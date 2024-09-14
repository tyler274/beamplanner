use crate::util::{Color, Sat, User, Vector3};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
    vec,
};

use rayon::prelude::*;

// In degrees
const MINIMUM_BEAM_ANGLE: f32 = 10.0;
const MAX_ALLOWABLE_BEAM_ANGLE: f32 = 45.0;
const MAX_ALLOWED_USERS: usize = 32;
const MAX_COLOR_OPTIONS: usize = 4;

type Map<K, V> = HashMap<K, V>;
type Set<K> = HashSet<K>;

type Users = Vec<Vector3>;
type Sats = Vec<Vector3>;

type UserSatsMap = Vec<Vec<Sat>>;
type SatsUsersMap = Vec<Vec<User>>;

type UserUserMap = Vec<Vec<User>>;
type SatUserInterferenceMap = Vec<Vec<Vec<User>>>;

type SolutionMap = Map<User, (Sat, Color)>;

type AvailaibleConnections = Set<(Color, User, Sat)>;

fn possible_connections(users: &Users, sats: &Sats) -> (UserSatsMap, SatsUsersMap) {
    let mut by_user: UserSatsMap = vec![Vec::with_capacity(sats.len()); users.len() + 1];
    let mut by_sat: SatsUsersMap = vec![Vec::with_capacity(users.len()); sats.len() + 1];

    for (sat_id, sat_pos) in sats.iter().enumerate() {
        for (user_id, user_pos) in users.iter().enumerate() {
            let angle = Vector3::zero().angle_between(user_pos, &(sat_pos - user_pos));
            if angle <= MAX_ALLOWABLE_BEAM_ANGLE {
                by_user.get_mut(user_id).unwrap().push(Sat(sat_id as u64));
                by_sat.get_mut(sat_id).unwrap().push(User(user_id as u64));
            }
        }
    }

    (by_user, by_sat)
}

fn get_interferences(
    users: &Users,
    sats: &Sats,
    conns_by_sat: &SatsUsersMap,
) -> SatUserInterferenceMap {
    // let mut by_sat_user: SatUserInterferenceMap = Default::default();
    let mut by_sat_user: Vec<Vec<Vec<User>>> =
        vec![vec![Vec::with_capacity(users.len()); users.len() + 1]; sats.len() + 1];

    for (sat_id, sat_users) in conns_by_sat.iter().enumerate() {
        // let mut angle_scratchpad: Vec<(usize, usize, usize, f32)> =
        //     Vec::with_capacity(sat_users.len() * sat_users.len());
        let angles: Vec<(&User, &User, f32)> = sat_users
            .par_iter()
            .map(|user_id| {
                let user_pos = users.get(user_id.0 as usize).unwrap();
                let angles = sat_users
                    .par_iter()
                    .map(|other_user_id| {
                        let other_user_pos = users.get(other_user_id.0 as usize).unwrap();
                        let angle = sats
                            .get(sat_id)
                            .unwrap()
                            .angle_between(user_pos, other_user_pos);
                        (user_id, other_user_id, angle)
                    })
                    .collect::<Vec<_>>();
                angles
            })
            .flatten()
            .collect();
        for angle in angles {
            if angle.2 < MINIMUM_BEAM_ANGLE {
                by_sat_user[sat_id][angle.1 .0 as usize].push(*angle.1);
            }
        }
    }

    by_sat_user
}

pub fn solve(users: &HashMap<User, Vector3>, sats: &HashMap<Sat, Vector3>) -> SolutionMap {
    let mut users_vec = vec![Vector3::zero(); users.len() + 1];
    for (user, pos) in users.iter() {
        users_vec[user.0 as usize] = *pos;
    }
    let mut sats_vec = vec![Vector3::zero(); sats.len() + 1];
    for (sat, pos) in sats.iter() {
        sats_vec[sat.0 as usize] = *pos;
    }

    let (conns_by_user, conns_by_sat) = possible_connections(&users_vec, &sats_vec);
    let interference_by_sat_user = get_interferences(&users_vec, &sats_vec, &conns_by_sat);

    let mut available_conns: AvailaibleConnections = Default::default();
    for (sat_id, sat_users) in conns_by_sat.iter().enumerate() {
        for user_id in sat_users {
            for i in 0..MAX_COLOR_OPTIONS {
                available_conns.insert((
                    Color::from_id(i as i32 + 1),
                    *user_id,
                    Sat(sat_id as u64),
                ));
            }
        }
    }

    let mut sat_conn_count = vec![0; sats.len() + 1];
    let mut solution: SolutionMap = Default::default();

    while available_conns.len() > 0 {
        let (color, user_id, sat_id) = *available_conns.iter().next().unwrap();
        available_conns.remove(&(color, user_id, sat_id));

        *sat_conn_count.get_mut(sat_id.0 as usize).unwrap() += 1;

        solution
            .entry(user_id)
            .and_modify(|e| {
                e.0 = sat_id;
                e.1 = color;
            })
            .or_insert((sat_id, color));

        // dont add interfering connections
        for user2_id in interference_by_sat_user
            .get(sat_id.0 as usize)
            .unwrap()
            .get(user_id.0 as usize)
            .unwrap_or(&Default::default())
        {
            available_conns.remove(&(color, *user2_id, sat_id));
        }

        // don't reconnect the same user
        for sat_id in conns_by_user.get(user_id.0 as usize).unwrap() {
            for i in 0..MAX_COLOR_OPTIONS {
                available_conns.remove(&(Color::from_id(i as i32 + 1), user_id, *sat_id));
            }
        }

        // if satellite is at capacity, drop its remaining possible connections
        if *sat_conn_count.get(sat_id.0 as usize).unwrap() >= MAX_ALLOWED_USERS {
            for user_id in conns_by_sat.get(sat_id.0 as usize).unwrap() {
                for i in 0..MAX_COLOR_OPTIONS {
                    available_conns.remove(&(Color::from_id(i as i32 + 1), *user_id, sat_id));
                }
            }
        }
    }
    solution
}

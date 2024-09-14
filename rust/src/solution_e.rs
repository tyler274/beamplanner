use crate::util::{Color, Sat, User, Vector3};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
};

// In degrees
const MINIMUM_BEAM_ANGLE: f32 = 10.0;
const MAX_ALLOWABLE_BEAM_ANGLE: f32 = 45.0;
const MAX_ALLOWED_USERS: usize = 32;
const MAX_COLOR_OPTIONS: usize = 4;

type Map<K, V> = HashMap<K, V>;
type Set<K> = HashSet<K>;

type Users = Map<User, Vector3>;
type Sats = Map<Sat, Vector3>;

type UserSatsMap = Map<User, Set<Sat>>;
type SatsUsersMap = Map<Sat, Set<User>>;

type UserUserMap = Map<User, Set<User>>;
type SatUserInterferenceMap = Map<Sat, UserUserMap>;

type SolutionMap = Map<User, (Sat, Color)>;

type AvailaibleConnections = Set<(Color, User, Sat)>;

fn possible_connections(users: &Users, sats: &Sats) -> (UserSatsMap, SatsUsersMap) {
    let mut by_user: UserSatsMap = Default::default();
    let mut by_sat: SatsUsersMap = Default::default();
    for (sat_id, sat_pos) in sats.iter() {
        for (user_id, user_pos) in users.iter() {
            let angle = Vector3::zero().angle_between(user_pos, &(sat_pos - user_pos));
            if angle <= MAX_ALLOWABLE_BEAM_ANGLE {
                by_user
                    .entry(*user_id)
                    .or_insert_with(Default::default)
                    .insert(*sat_id);
                by_sat
                    .entry(*sat_id)
                    .or_insert_with(Default::default)
                    .insert(*user_id);
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
    let mut by_sat_user: SatUserInterferenceMap = Default::default();

    for (sat_id, sat_users) in conns_by_sat.iter() {
        let mut interferences: UserUserMap = Default::default();
        for user_id in sat_users {
            let user_pos = users.get(user_id).unwrap();
            for other_user_id in sat_users {
                if user_id != other_user_id {
                    let other_user_pos = users.get(other_user_id).unwrap();
                    let angle = sats
                        .get(sat_id)
                        .unwrap()
                        .angle_between(user_pos, other_user_pos);
                    // TODO: Hoist this outside the loop to avoid a branch during parallel computation
                    if angle < MINIMUM_BEAM_ANGLE {
                        interferences
                            .entry(*user_id)
                            .or_insert_with(Default::default)
                            .insert(*other_user_id);
                        // interferences
                        //     .entry(*other_user_id)
                        //     .or_insert_with(Default::default)
                        //     .insert(*user_id);
                    }
                }
            }
        }
        for user_id in sat_users {
            let user_pos = users.get(user_id).unwrap();
            for other_user_id in sat_users {
                if user_id != other_user_id {
                    let other_user_pos = users.get(other_user_id).unwrap();
                    let angle = sats
                        .get(sat_id)
                        .unwrap()
                        .angle_between(user_pos, other_user_pos);
                    // TODO: Hoist this outside the loop to avoid a branch during parallel computation
                    if angle < MINIMUM_BEAM_ANGLE {
                        interferences
                            .entry(*user_id)
                            .or_insert_with(Default::default)
                            .insert(*other_user_id);
                        // interferences
                        //     .entry(*other_user_id)
                        //     .or_insert_with(Default::default)
                        //     .insert(*user_id);
                    }
                }
            }
        }
        // Hoist the angle check here
        by_sat_user.insert(*sat_id, interferences);
    }

    by_sat_user
}

pub fn solve(users: &HashMap<User, Vector3>, sats: &HashMap<Sat, Vector3>) -> SolutionMap {
    let mut solution: SolutionMap = Default::default();

    // let users = HashMap::from_iter(users.iter().map(|(k, v)| (*k, *v)));
    // let sats = HashMap::from_iter(sats.iter().map(|(k, v)| (*k, *v)));

    let (conns_by_user, conns_by_sat) = possible_connections(&users, &sats);
    let interference_by_sat_user = get_interferences(&users, &sats, &conns_by_sat);

    let mut available_conns: AvailaibleConnections = Default::default();
    for (sat_id, sat_users) in conns_by_sat.iter() {
        for user_id in sat_users {
            for i in 0..MAX_COLOR_OPTIONS {
                available_conns.insert((Color::from_id(i as i32 + 1), *user_id, *sat_id));
            }
        }
    }

    let mut solution_by_sat: SatsUsersMap = Default::default();

    while available_conns.len() > 0 {
        let (color, user_id, sat_id) = *available_conns.iter().next().unwrap();
        available_conns.remove(&(color, user_id, sat_id));

        solution_by_sat
            .entry(sat_id)
            .or_insert_with(Default::default)
            .insert(user_id);

        solution
            .entry(user_id)
            .and_modify(|e| {
                e.0 = sat_id;
                e.1 = color;
            })
            .or_insert((sat_id, color));

        // dont add interfering connections
        for user2_id in interference_by_sat_user
            .get(&sat_id)
            .unwrap()
            .get(&user_id)
            .unwrap_or(&Default::default())
        {
            available_conns.remove(&(color, *user2_id, sat_id));
        }

        // don't reconnect the same user
        for sat_id in conns_by_user.get(&user_id).unwrap() {
            for i in 0..MAX_COLOR_OPTIONS {
                available_conns.remove(&(Color::from_id(i as i32 + 1), user_id, *sat_id));
            }
        }

        // if satellite is at capacity, drop its remaining possible connections
        if solution_by_sat.get(&sat_id).unwrap().len() >= MAX_ALLOWED_USERS {
            for user_id in conns_by_sat.get(&sat_id).unwrap() {
                for i in 0..MAX_COLOR_OPTIONS {
                    available_conns.remove(&(Color::from_id(i as i32 + 1), *user_id, sat_id));
                }
            }
        }
    }
    solution
}

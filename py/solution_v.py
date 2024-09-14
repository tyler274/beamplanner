import collections
import itertools
import math
import sys

from util import Color, Vector3

# utils

def vlen(v):
    return math.sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])

def normalize(v):
    length = vlen(v)
    return (v[0]/length, v[1]/length, v[2]/length)

def dot(a, b):
    return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]

def sub(a, b):
    return (a[0]-b[0], a[1]-b[1], a[2]-b[2])

def angle(a, b):
    return math.acos(dot(normalize(a), normalize(b)))

def rad2deg(x):
    return x / math.pi * 180

# problem

def parse(path):
    min = None
    sats = {}
    users = {}
    with open(path, 'r') as f:
         min = float(f.readline().split(" ")[1])
         for line in f:
             kind, id, x, y, z = line.strip().split(" ")
             id = int(id)
             x, y, z = float(x), float(y), float(z)
             if kind == "user":
                 users[id] = (x, y, z)
             elif kind == "sat":
                 sats[id] = (x, y, z)
             else:
                 raise Exception("couldn't parse", line)
    return min, sats, users

# solution

# Connection = (Color, SatId, UserId)

def possible_connections(sats, users, colors, max_user_angle):
    by_user = [[] for _ in range(len(users)+1)]
    by_sat = [[] for _ in range(len(sats)+1)]
    for sat_id, sat_pos in sats.items():
        for user_id, user_pos in users.items():
            a = rad2deg(angle(user_pos, sub(sat_pos, user_pos)))
            if a < max_user_angle:
                by_user[user_id].append(sat_id)
                by_sat[sat_id].append(user_id)
    return by_sat, by_user

def get_interferences(sats, users, conns_by_sat, min_beam_separation):
    by_sat_user = [[[] for _ in range(len(users)+1)] for _ in range(len(sats)+1)]
    for sat_id, sat_users in enumerate(conns_by_sat):
        for user1_id, user2_id in itertools.combinations(sat_users, 2):
            if rad2deg(angle(
                sub(users[user1_id], sats[sat_id]),
                sub(users[user2_id], sats[sat_id])
            )) < min_beam_separation:
                by_sat_user[sat_id][user1_id].append(user2_id)
                by_sat_user[sat_id][user2_id].append(user1_id)
    return by_sat_user


def _solve(users, sats, colors=4, max_user_angle=45, min_beam_separation=10, max_conn_per_sat=32):
    conns_by_sat, conns_by_user = possible_connections(sats, users, colors=colors, max_user_angle=max_user_angle)
    interference_by_sat_user = get_interferences(sats, users, conns_by_sat, min_beam_separation)

    available_conns = set()
    for sat_id, sat_users in enumerate(conns_by_sat):
        for user_id in sat_users:
            for i in range(colors):
                available_conns.add((i, sat_id, user_id))
    sat_conn_count = [0 for _ in range(len(sats)+1)]
    solution = {} # user -> [sat, color]
    while len(available_conns) > 0:
        color, sat_id, user_id = available_conns.pop()
        sat_conn_count[sat_id] += 1
        solution[user_id] = (sat_id, Color(color+1))
        # dont add interfering connections
        for user2_id in interference_by_sat_user[sat_id][user_id]:
            available_conns.discard((color, sat_id, user2_id))
        # don't reconnect the same user
        for sat2_id in conns_by_user[user_id]:
            for i in range(colors):
                available_conns.discard((i, sat2_id, user_id))
        # if satellite is at capacity, drop its remaining possible connections
        if sat_conn_count[sat_id] >= max_conn_per_sat:
            for i in range(colors):
                for user2_id in conns_by_sat[sat_id]:
                    available_conns.discard((i, sat_id, user2_id))
    return solution

def solve(users, sats):
    print("solving")
    return _solve(
        {i: (v.x,v.y,v.z) for i,v in users.items()},
        {i: (v.x,v.y,v.z) for i,v in sats.items()},
    )
    print("solved")

def main(in_path, out_path):
    min, sats, users = parse(in_path)
    #print(min, sats, users)

    solution = solve(users, sats)


    #print("write", out_path)

if __name__== "__main__":
   main(sys.argv[1], sys.argv[2])
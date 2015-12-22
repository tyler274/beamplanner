#pragma once

#include "math.h"

#define PI 3.14159265359f

// Type aliases for API readability.
typedef int Sat;
typedef int User;
typedef char Color;

typedef struct
{
    User user;
    Sat sat;
    Color color;
} Beam;

typedef struct
{
    Beam beams[32];
    int num_beams;
} Satellite;

// A 3-dimensional vector.
typedef struct
{
    float x;
    float y;
    float z;
} Vector3;

// Return the sum of two vectors.
static inline Vector3 add(const Vector3 a, const Vector3 b)
{
    Vector3 r = {a.x + b.x, a.y + b.y, a.z + b.z};
    return r;
}

// Return the difference of two vectors (a-b).
static inline Vector3 sub(const Vector3 a, const Vector3 b)
{
    Vector3 r = {a.x - b.x, a.y - b.y, a.z - b.z};
    return r;
}

// Return the vector's magnitude.
static inline float length(const Vector3 v)
{
    return sqrtf(v.x * v.x + v.y * v.y + v.z * v.z);
}

// Return the vector normalized to a length of 1.
static inline Vector3 normalize(const Vector3 v)
{
    const float m = length(v);
    Vector3 r = {v.x / m, v.y / m, v.z / m};
    return r;
}

// Return the dot product of two vectors.
static inline float dot(const Vector3 a, const Vector3 b)
{
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

// Return the angle in degrees between (a - b) and (c - b):
//
//     a       c
//      \     /
//       \.·./
//        \ /
//         b
//
static inline float angle_between(Vector3 a, Vector3 b, Vector3 c)
{
    const Vector3 m = normalize(sub(a, b));
    const Vector3 n = normalize(sub(c, b));
    const float r = dot(m, n);

    return acosf(r) * 180 / PI;
}

// Generate a set of legal beams to cover as many users as possible.
void solve(const Vector3* users, const int num_users, const Vector3* sats, const int num_sats);

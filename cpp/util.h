#pragma once

#include <cmath>
#include <map>
#include <vector>

/**
 * Constants.
 */
#define PI 3.14159265359f

/**
 * Type aliases for API readability.
 */
using Sat = int;
using User = int;
using Color = char;

/**
 * Valid beam colors.
 */
constexpr Color colors[] = {'A', 'B', 'C', 'D'};

/**
 * A 3-dimensional vector.
 */
struct Vector3
{
    /**
     * Coordinates.
     */
    float x = 0.0;
    float y = 0.0;
    float z = 0.0;

    /**
     * Zero constructor.
     */
    Vector3() = default;

    /**
     * Return the sum of two vectors.
     */
    friend Vector3 operator+(Vector3 a, Vector3 b)
    {
        return Vector3{a.x + b.x, a.y + b.y, a.z + b.z};
    }

    /**
     * Return the difference of two vectors.
     */
    friend Vector3 operator-(Vector3 a, Vector3 b)
    {
        return Vector3{a.x - b.x, a.y - b.y, a.z - b.z};
    }

    /**
     * Return the vector's magnitude (length).
     */
    float mag() const
    {
        return sqrt(x * x + y * y + z * z);
    }

    /**
     * Return the vector normalized to length 1.
     *
     * WARNING: Undefined for zero-length vectors.
     */
    Vector3 unit() const
    {
        const float m = mag();
        return Vector3{x / m, y / m, z / m};
    }

    /**
     * Return the dot product of two vectors.
     */
    float dot(Vector3 a) const
    {
        return x * a.x + y * a.y + z * a.z;
    }

    /**
     * Return the angle in degrees between (a - *this) and (c - *this):
     *
     *     a       c
     *      \     /
     *       \.·./
     *        \ /
     *       *this
     *
     */
    float angle_between(Vector3 a, Vector3 c) const
    {
        const Vector3 m = (a - *this).unit();
        const Vector3 n = (c - *this).unit();
        const float r = m.dot(n);

        return acos(r) * 180 / PI;
    }
};

/**
 * Generate a set of legal beams to cover as many users as possible.
 */
std::map<User, std::pair<Sat, Color>> solve(
    const std::map<User, Vector3>& users,
    const std::map<Sat, Vector3>& sats);

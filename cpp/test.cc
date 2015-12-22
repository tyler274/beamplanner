#include "test_util.h"
#include "util.h"

using namespace std::chrono_literals;

std::istream& operator>>(std::istream& is, Vector3& vector)
{
    return is >> vector.x >> vector.y >> vector.z;
}

void check_color(Color color)
{
    for (const Color c : colors)
    {
        if (color == c)
        {
            return;
        }
    }

    FAIL("Invalid color: '%c'", color);
}

int main(int argc, char** argv)
{
    CHECK(argc == 3, "USAGE: %s OUT_PATH TEST_CASE", argv[0]);

    /*
     * Parse test case.
     */
    float min_coverage = 1.0;
    std::map<User, Vector3> users;
    std::map<Sat, Vector3> sats;
    for (const auto& [line_number, line] : read_file(argv[2]))
    {
        std::istringstream tokens(line, std::istringstream::in);
        int id = 0;
        Vector3 pos;
        std::string token;
        tokens >> token;
        if (token == "min_coverage")
        {
            tokens >> min_coverage;
        }
        else
        {
            tokens >> id >> pos;
            if (token == "sat")
            {
                sats[id] = pos;
            }
            else if (token == "user")
            {
                users[id] = pos;
            }
            else
            {
                FAIL("Invalid token '%s'.", token.c_str());
            }
        }
    }
    printf(
        GRAY "Scenario: " RESET "%.2f%% coverage (%zu users, %zu sats)" RESET "\n",
        static_cast<double>(100 * min_coverage),
        users.size(),
        sats.size());

    /*
     * Run candidate code.
     */
    const auto start = std::chrono::high_resolution_clock::now();
    const std::map<User, std::pair<Sat, Color>> solution = solve(users, sats);
    const auto time = std::chrono::high_resolution_clock::now() - start;
    const float duration_s = time / 1ns / 1e9f;
    const float coverage = 1.0f * solution.size() / users.size();
    printf(
        GRAY "Solution: " RESET BOLD "%s%.2f%%" RESET " coverage (%zu users) in %s" BOLD
             "%.2fs" RESET "\n",
        coverage >= min_coverage ? GREEN : RED,
        static_cast<double>(100.0f * coverage),
        solution.size(),
        time > 60s       ? RED
        : time > 60s / 2 ? YELLOW
                         : GREEN,
        static_cast<double>(duration_s));

    /*
     * Check solution.
     */
    std::map<Sat, std::vector<std::pair<Color, User>>> beams;
    for (const auto& it : solution)
    {
        const User user = it.first;
        const Sat sat = it.second.first;
        const Color color = it.second.second;
        const Vector3 user_pos = users.at(user);
        const Vector3 sat_pos = sats.at(sat);
        check_color(color);

        const float angle = acos(user_pos.unit().dot((sat_pos - user_pos).unit())) / PI * 180;
        CHECK(
            angle <= 45,
            "User %d cannot see satellite %d (%.2f degrees from vertical)",
            user,
            sat,
            static_cast<double>(angle));

        beams[sat].emplace_back(std::make_pair(color, user));
    }
    for (const auto& it : beams)
    {
        const Sat sat = it.first;
        const Vector3 sat_pos = sats.at(sat);
        const size_t sat_beams = it.second.size();
        CHECK(
            sat_beams <= 32,
            "Satellite %d cannot serve more than 32 users (%zu assigned)",
            sat,
            sat_beams);

        for (const std::pair<Color, User>& u1 : it.second)
        {
            const Color color1 = u1.first;
            const User user1 = u1.second;

            for (const std::pair<Color, User>& u2 : it.second)
            {
                const Color color2 = u2.first;
                const User user2 = u2.second;

                if (color1 == color2 && user1 != user2)
                {
                    const Vector3 user1_pos = users.at(user1);
                    const Vector3 user2_pos = users.at(user2);
                    const float angle = sat_pos.angle_between(user1_pos, user2_pos);
                    CHECK(
                        angle >= 10.0f,
                        "Users %d and %d on satellite %d color %c are too close (%.2f degrees)",
                        user1,
                        user2,
                        sat,
                        color1,
                        static_cast<double>(angle));
                }
            }
        }
    }
    CHECK(coverage >= min_coverage, "Too few users served");

    /*
     * Output stats.
     */
    FILE* out = fopen(argv[1], "a");
    CHECK(out, "Error opening output %s: %s (errno=%d)", argv[1], strerror(errno), errno);
    fprintf(out, "%-44s %6.2f%% %6.2fs\n", argv[2], static_cast<double>(100.0f * coverage), static_cast<double>(duration_s));
}

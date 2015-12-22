#include "test_util.h"
#include "util.h"

#include "string.h"
#include "math.h"
#include "time.h"
#include "errno.h"

int main(int argc, char** argv)
{
    CHECK(argc == 3, "USAGE: %s OUT_PATH TEST_CASE", argv[0]);

    // Confirm input file exists.
    const char *input_filename = argv[2];
    FILE *input_file;
    CHECK(input_file = fopen(input_filename, "r"), "%s test file does not exist!\n", input_filename);

    // Parse test case.
    int num_users = 0;
    int num_sats = 0;
    double min_coverage = 0.0;
    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, input_file) != -1)
    {
        if (strstr(line, "user"))
        {
            num_users++;
        }
        else if (strstr(line, "sat"))
        {
            num_sats++;
        }
        else if(strstr(line, "min_coverage"))
        {
            min_coverage = atof(strstr(line, " "));
        }
    }
    printf(
        GRAY "Scenario: " RESET "%.2f%% coverage (%d users, %d sats)" RESET "\n",
        (100 * min_coverage),
        num_users,
        num_sats);

    // Allocate memory and parse for real.
    Vector3* users = (Vector3*)malloc(num_users * sizeof(Vector3));
    Vector3* sats = (Vector3*)malloc(num_sats * sizeof(Vector3));
    int tmp;
    int cur_user = 0;
    int cur_sat = 0;
    Vector3 v = {0.0f, 0.0f, 0.0f};
    fseek(input_file, 0, SEEK_SET);
    while (getline(&line, &len, input_file) != -1)
    {
        if (strstr(line, "user"))
        {
            CHECK(4  == sscanf(line, "user %d %f %f %f", &tmp, &v.x, &v.y, &v.z),
                    "Error reading in user.");
            CHECK(cur_user < num_users, "user ID (%d) larger than num users (%d).", cur_user, num_users);
            users[cur_user].x = v.x;
            users[cur_user].y = v.y;
            users[cur_user].z = v.z;
            cur_user++;
        }
        else if (strstr(line, "sat"))
        {
            CHECK(4 == sscanf(line, "sat %d %f %f %f", &tmp, &v.x, &v.y, &v.z),
                    "Error reading in sat.");
            CHECK(cur_sat < num_sats, "sat ID (%d) larger than num sats (%d).", cur_sat, num_sats);
            sats[cur_sat].x = v.x;
            sats[cur_sat].y = v.y;
            sats[cur_sat].z = v.z;
            cur_sat++;
        }
    }

    // Make copies to pass to candidate code.
    Vector3* users_cp = (Vector3*)malloc(num_users * sizeof(Vector3));
    Vector3* sats_cp = (Vector3*)malloc(num_sats * sizeof(Vector3));
    memcpy(users_cp, users, num_users * sizeof(Vector3));
    memcpy(sats_cp, sats, num_sats * sizeof(Vector3)); 

    // Delete old solution file.
    remove("solution.txt");

    // Time and run candidates code.
    struct timespec start, end;
    CHECK(clock_gettime(CLOCK_MONOTONIC, &start) >= 0, "Error reading start time.");
    solve(users_cp, num_users, sats_cp, num_sats);
    CHECK(clock_gettime(CLOCK_MONOTONIC, &end) >= 0, "Error reading end time.");
    const double duration_s = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;

    // Parse results.
    FILE *file;
    CHECK(file = fopen("solution.txt", "r"), "solution.txt file not found.");
    Beam* beams = (Beam*)malloc(num_users*sizeof(Beam));
    CHECK(beams, "Error mallocating beams.");
    memset(beams, 0, (num_users*sizeof(Beam)));
    int b = 0;
    for(; getline(&line, &len, file) != -1; b++)
    {
        User u;
        Sat s;
        Color c;
        CHECK(3 == sscanf(line, "%d %d %c", &u, &s, &c), "Error reading in solution.");
        CHECK(u < num_users, "'%d' is not a valid user id.", u);
        CHECK(s < num_sats, "'%d' is not a valid sat id.", s);
        CHECK(c >= 'A' && c <= 'D', "'%c' is not a valid beam color.", c);

        beams[b].user = u;
        beams[b].sat = s;
        beams[b].color = c;
    }
    const int total_beams = b;
    const double coverage = 1.0 * total_beams / num_users;
    printf(
        GRAY "Solution: " RESET BOLD "%s%.2f%%" RESET " coverage (%d users) in %s" BOLD
             "%.2fs" RESET "\n",
        coverage >= min_coverage ? GREEN : RED,
        (double)(100.0 * coverage),
        total_beams,
        duration_s > 60       ? RED
        : duration_s > 60 / 2 ? YELLOW
                         : GREEN,
        (double)duration_s);

    // Sort results by sat and check the 45 deg constraint.
    Satellite* satellites = (Satellite*)malloc(num_sats*sizeof(Satellite));
    CHECK(satellites, "Error mallocing satellites.");
    memset(satellites, 0, num_sats*sizeof(Satellite));
    for (int beam = 0; beam < total_beams; beam++)
    {
        const Sat sat_id = beams[beam].sat;
        const User user_id = beams[beam].user;
        const Color color = beams[beam].color;

        const int n = satellites[sat_id].num_beams;
        CHECK(n < 32, "Something went wrong.");
        satellites[sat_id].beams[n].user = user_id;
        satellites[sat_id].beams[n].color = color;
        satellites[sat_id].num_beams++;

        // Check angle between user and satellite;
        const Vector3 user_pos = users[user_id];
        const Vector3 sat_pos = sats[sat_id];
        const float angle =
            180.0f * acosf(dot(normalize(user_pos), normalize(sub(sat_pos,user_pos)))) / PI;
        CHECK(
            angle <= 45,
            "User %d cannot see satellite %d (%.2f degrees from vertical)",
            user_id,
            sat_id,
            (double)angle);
    }

    // Iterate through each sat and check the angles between like colors.
    for (Sat sat = 0; sat < num_sats; sat++)
    {
        const Vector3 sat_pos = sats[sat];
        const int sat_beams = satellites[sat].num_beams;
        CHECK(
            sat_beams <= 32,
            "Satellite %d cannot serve more than 32 users (%d assigned)",
            sat,
            sat_beams);
        for (int u1 = 0; u1 < sat_beams; u1++)
        {
            const Color color1 = satellites[sat].beams[u1].color;
            const User user1 = satellites[sat].beams[u1].user;

            for (int u2 = u1 + 1; u2 < sat_beams; u2++)
            {
                const Color color2 = satellites[sat].beams[u2].color;
                const User user2 = satellites[sat].beams[u2].user;
                CHECK(user1 != user2, "Something went wrong... %d %d", user1, user2);
                if (color1 == color2)
                {
                    const Vector3 user1_pos = users[user1];
                    const Vector3 user2_pos = users[user2];
                    const float angle =
                        180.0f *
                        acosf(dot(normalize(sub(user1_pos, sat_pos)), normalize(sub(user2_pos,sat_pos)))) /
                        PI;
                    CHECK(
                        angle >= 10.0f,
                        "Users %d and %d on satellite %d color %c are too close (%.2f degrees)",
                        user1,
                        user2,
                        sat,
                        color1,
                        (double)angle);
                }
            }
        }
    }

    // Check coverage.
    CHECK(coverage >= min_coverage, "Too few users served");

    // Output stats.
    FILE* out = fopen(argv[1], "a");
    CHECK(out, "Error opening output %s: %s (errno=%d)", argv[1], strerror(errno), errno);
    fprintf(out, "%-44s %6.2f%% %6.2fs\n", argv[2], (double)(100.0 * coverage), (double)(duration_s));

    // Clean up.
    free(line);
    free(users);
    free(users_cp);
    free(sats);
    free(sats_cp);
    free(beams);
    free(satellites);
    fclose(input_file);
    fclose(file);
    fclose(out);
}

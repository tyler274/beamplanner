static class StarlinkTest
{
    static TimeSpan TIMEOUT = TimeSpan.FromSeconds(600);


    static void Main(string[] args)
    {
        TestUtil.Check(args.Length == 2, $"USAGE: {AppDomain.CurrentDomain.FriendlyName} OUT_PATH TEST_CASE");

        string outPath = args[0];
        string testCase = args[1];

        var scenario = new Scenario(testCase);

        Console.WriteLine($"Scenario: {scenario.MinCoverage:P2} coverage ({scenario.Users.Count} users, {scenario.Sats.Count} sats)");

        var start = DateTime.Now;
        var solution = StarlinkSolution.Solve(scenario.Users, scenario.Sats);
        var duration = DateTime.Now - start;
        var covered = (double)solution.Count / scenario.Users.Count;

        Console.WriteLine(
            $"Solution: {(covered >= scenario.MinCoverage ? TestUtil.GREEN : TestUtil.RED)}{(covered * 100):F2}%{TestUtil.RESET} " +
            $"coverage ({solution.Count} users) " +
            $"in {(duration > TIMEOUT ? TestUtil.RED : duration > TIMEOUT / 2 ? TestUtil.YELLOW : TestUtil.GREEN)}{duration.TotalSeconds:F2}s{TestUtil.RESET}"
        );

        using (var writer = new StreamWriter(outPath, true))
        {
            writer.WriteLine($"{testCase,-44} {(covered * 100),6:F2}% {duration.TotalSeconds,6:F2}s");
        }

        TestUtil.Check(duration < TIMEOUT, "Took too long to produce a solution");

        scenario.Check(solution);
    }
}

class Scenario
{
    public Dictionary<Sat, Vector3> Sats { get; }
    public Dictionary<User, Vector3> Users { get; }
    public double MinCoverage { get; }

    public Scenario(string path)
    {
        Sats = new Dictionary<Sat, Vector3>();
        Users = new Dictionary<User, Vector3>();
        MinCoverage = 1.0;

        foreach (var line in File.ReadLines(path))
        {
            var parts = line.Split('#')[0].Trim().Split();
            if (parts.Length == 0)
            {
                continue;
            }

            switch (parts[0])
            {
                case "min_coverage":
                    MinCoverage = double.Parse(parts[1]);
                    break;
                case "sat":
                    Sats[new Sat(int.Parse(parts[1]))] = new Vector3(float.Parse(parts[2]), float.Parse(parts[3]), float.Parse(parts[4]));
                    break;
                case "user":
                    Users[new User(int.Parse(parts[1]))] = new Vector3(float.Parse(parts[2]), float.Parse(parts[3]), float.Parse(parts[4]));
                    break;
                default:
                    TestUtil.Fail($"Invalid token: {parts[0]}");
                    break;
            }
        }
    }

    public void Check(Dictionary<User, (Sat, Color)> solution)
    {
        var beams = new Dictionary<Sat, List<(Color, User)>>();

        foreach (var (user, (sat, color)) in solution)
        {
            var userPos = Users[user];
            var satPos = Sats[sat];

            TestUtil.Check(Enum.IsDefined(typeof(Color), color), $"Invalid color: {color}");

            var angle = Math.Acos(userPos.Unit().Dot((satPos - userPos).Unit()));
            TestUtil.Check(angle <= Math.PI / 4, $"User {user} cannot see satellite {sat} ({angle * 180 / Math.PI:F2} degrees from vertical)");

            if (!beams.ContainsKey(sat))
            {
                beams[sat] = new List<(Color, User)>();
            }

            beams[sat].Add((color, user));
        }

        foreach (var (sat, satBeams) in beams)
        {
            var satPos = Sats[sat];

            TestUtil.Check(satBeams.Count <= 32, $"Satellite {sat} cannot serve more than 32 users ({satBeams.Count} assigned)");

            foreach (var ((color1, user1), (color2, user2)) in satBeams.SelectMany((x, i) => satBeams.Skip(i + 1).Select(y => (x, y))))
            {
                if (color1 == color2 && user1 != user2)
                {
                    var user1Pos = Users[user1];
                    var user2Pos = Users[user2];

                    var angle = satPos.AngleBetween(user1Pos, user2Pos);
                    TestUtil.Check(angle >= 10.0, $"Users {user1} and {user2} on satellite {sat} {color1} are too close ({angle:F2} degrees)");
                }
            }
        }

        var coverage = (double)solution.Count / Users.Count;
        TestUtil.Check(coverage >= MinCoverage, "Too few users served");
    }
}

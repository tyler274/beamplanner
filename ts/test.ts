#!/usr/bin/env ts-node

import fs from "fs";
import {
  BOLD,
  check,
  fail,
  GRAY,
  GREEN,
  readLines,
  RED,
  RESET,
  YELLOW,
} from "./test-util";

import { solve } from "./solution";
import {
  Color,
  Vector3,
  Sat,
  User,
  RAD_TO_DEG,
  DefaultMap,
  COLORS,
} from "./util";

const TIMEOUT = 600 * 1000;

class Scenario {
  sats: Map<Sat, Vector3>;
  users: Map<User, Vector3>;
  min_coverage: number;

  constructor(path: string) {
    this.sats = new Map();
    this.users = new Map();
    this.min_coverage = 1.0;

    for (let line of readLines(path)) {
      line = line.split("#")[0].trim();
      if (line === "") {
        continue;
      }
      const parts = line.split(" ");
      if (parts[0] === "min_coverage") {
        this.min_coverage = parseFloat(parts[1]);
      } else if (parts[0] === "sat") {
        this.sats.set(
          parseInt(parts[1]) as Sat,
          new Vector3(
            parseFloat(parts[2]),
            parseFloat(parts[3]),
            parseFloat(parts[4])
          )
        );
      } else if (parts[0] === "user") {
        this.users.set(
          parseInt(parts[1]) as User,
          new Vector3(
            parseFloat(parts[2]),
            parseFloat(parts[3]),
            parseFloat(parts[4])
          )
        );
      } else {
        fail(`Invalid token: ${parts[0]}`);
      }
    }
  }

  check(solution: Map<User, [Sat, Color]>) {
    const beams = new DefaultMap<Sat, [Color, User][]>(() => []);

    for (const [user, [sat, color]] of solution.entries()) {
      const user_pos = this.users.get(user);
      const sat_pos = this.sats.get(sat);

      check(user_pos, `Invalid user`);
      check(sat_pos, `Invalid sat`);
      check(COLORS.has(color), `Invalid color: ${color}`);

      const angle =
        Math.acos(user_pos.unit().dot(sat_pos.sub(user_pos).unit())) *
        RAD_TO_DEG;

      check(
        angle <= 45,
        `User ${user} cannot see satellite ${sat} (${angle.toFixed(
          2
        )} degrees from vertical)`
      );

      beams.get(sat).push([color, user]);
    }

    for (const [sat, sat_beams] of beams.entries()) {
      const sat_pos = this.sats.get(sat);

      check(sat_pos, `Invalid sat`);
      check(
        sat_beams.length <= 32,
        `Satellite ${sat} cannot serve more than 32 users (${sat_beams.length} assigned)`
      );

      for (const [color1, user1] of sat_beams) {
        for (const [color2, user2] of sat_beams) {
          if (color1 === color2 && user1 !== user2) {
            const user1_pos = this.users.get(user1);
            const user2_pos = this.users.get(user2);
            check(user1_pos && user2_pos, `Invalid user`);
            const angle = sat_pos.angleBetween(user1_pos, user2_pos);
            check(
              angle >= 10.0,
              `Users ${user1} and ${user2} on satellite ${sat} ${color1} are too close (${angle.toFixed(
                2
              )} degrees)`
            );
          }
        }
      }
    }

    const coverage = (1.0 * solution.size) / this.users.size;
    check(coverage >= this.min_coverage, "Too few users served");
  }
}

export function main(argv: string[]) {
  check(argv.length === 4, `USAGE: test.ts OUT_PATH TEST_CASE`);

  const outPath = argv[2];
  const testCase = argv[3];

  const scenario = new Scenario(testCase);

  console.log(
    [
      `${GRAY}Scenario:${RESET}`,
      `${(100 * scenario.min_coverage).toFixed(2)}%`,
      `coverage (${scenario.users.size} users, ${scenario.sats.size} sats)`,
    ].join(" ")
  );

  const start = Date.now();
  const solution = solve(scenario.users, scenario.sats);
  const duration = Date.now() - start;
  const covered = (1.0 * solution.size) / scenario.users.size;

  console.log(
    [
      `${GRAY}Solution:${RESET}`,
      `${BOLD}${covered >= scenario.min_coverage ? GREEN : RED}${
        (100.0 * solution.size) / scenario.users.size
      }%${RESET}`,
      `coverage (${solution.size} users)`,
      `${
        duration > TIMEOUT ? RED : duration > TIMEOUT / 2 ? YELLOW : GREEN
      }in ${(duration / 1000).toFixed(2)}s${RESET}`,
    ].join(" ")
  );

  fs.appendFileSync(
    outPath,
    testCase.padEnd(44) +
      " " +
      (100 * covered).toFixed(2).padStart(6) +
      "% " +
      (duration / 1000).toFixed(2).padStart(6) +
      "s\n"
  );

  check(duration < TIMEOUT, "Took too long to produce a solution");

  scenario.check(solution);
}

if (require.main === module) {
  main(process.argv);
}

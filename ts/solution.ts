import { Color, Sat, User, Vector3 } from "./util";

export function solve(
  users: Map<User, Vector3>,
  sats: Map<Sat, Vector3>
): Map<User, [Sat, Color]> {
  // Assign users to satellites respecting all constraints.

  const solution = new Map<User, [Sat, Color]>();

  // TODO: Implement.

  return solution;
}

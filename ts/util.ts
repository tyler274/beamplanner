export type Color = "A" | "B" | "C" | "D";
export type Sat = Opaque<number, "Sat">;
export type User = Opaque<number, "User">;

export const COLORS = new Set<Color>(["A", "B", "C", "D"]);

// Helper for generating Opaque types.
type Opaque<T, K> = T & { __opaque__: K };

export class Vector3 {
  x: number;
  y: number;
  z: number;

  constructor(x: number, y: number, z: number) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  toString(): string {
    return `Vector3(${this.x}, ${this.y}, ${this.z})`;
  }

  add(other: Vector3): Vector3 {
    return new Vector3(this.x + other.x, this.y + other.y, this.z + other.z);
  }

  sub(other: Vector3): Vector3 {
    return new Vector3(this.x - other.x, this.y - other.y, this.z - other.z);
  }

  mag(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
  }

  unit(): Vector3 {
    const m = this.mag();
    return new Vector3(this.x / m, this.y / m, this.z / m);
  }

  dot(other: Vector3): number {
    return this.x * other.x + this.y * other.y + this.z * other.z;
  }

  angleBetween(a: Vector3, c: Vector3): number {
    const m = a.sub(this).unit();
    const n = c.sub(this).unit();
    const r = m.dot(n);

    return Math.acos(r) * RAD_TO_DEG;
  }
}

export const RAD_TO_DEG = 180 / Math.PI;

export class DefaultMap<K, V> extends Map {
  createDefault: () => V;
  constructor(createDefault: () => V) {
    super();
    this.createDefault = createDefault;
  }
  get(key: K): V {
    if (!this.has(key)) {
      this.set(key, this.createDefault());
    }
    return super.get(key);
  }
}

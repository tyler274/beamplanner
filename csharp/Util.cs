using System;

public enum Color
{
    A = 1,
    B = 2,
    C = 3,
    D = 4
}

public class Vector3
{
    private readonly float x;
    private readonly float y;
    private readonly float z;

    public Vector3(float x, float y, float z)
    {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public override string ToString()
    {
        return $"Vector3({x:.3}, {y:.3}, {z:.3})";
    }

    public static Vector3 operator +(Vector3 a, Vector3 b)
    {
        return new Vector3(a.x + b.x, a.y + b.y, a.z + b.z);
    }

    public static Vector3 operator -(Vector3 a, Vector3 b)
    {
        return new Vector3(a.x - b.x, a.y - b.y, a.z - b.z);
    }

    public float Mag()
    {
        return (float)Math.Sqrt(x * x + y * y + z * z);
    }

    public Vector3 Unit()
    {
        float m = Mag();
        return new Vector3(x / m, y / m, z / m);
    }

    public float Dot(Vector3 other)
    {
        return x * other.x + y * other.y + z * other.z;
    }

    public float AngleBetween(Vector3 a, Vector3 c)
    {
        Vector3 m = (a - this).Unit();
        Vector3 n = (c - this).Unit();
        float r = m.Dot(n);
        return (float)Math.Acos(r) * 180f / (float)Math.PI;
    }
}



public class Sat
{
    private readonly float value;
    public Sat(int value)
    {
        this.value = value;
    }

    override public string ToString()
    {
        return $"Sat({value})";
    }
}

public class User
{
    private readonly float value;
    public User(int value)
    {
        this.value = value;
    }

    override public string ToString()
    {
        return $"User({value})";
    }
}

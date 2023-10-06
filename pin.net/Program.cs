using System.Text;

if (args.Length < 1 || args[0] is not { } max)
{
    Console.Error.WriteLine("Create a 5 digit pin like: `pin 5`");
    return -1;
}

if (!int.TryParse(max, out var n))
{
    Console.Error.WriteLine($"Not a number: {max}");
    return -1;
}

var sb = new StringBuilder(n);

var rng = new Random();
for (int i = 0; i < n; i++)
{
    sb.Append(rng.Next(0, 10));
}

Console.WriteLine(sb.ToString());
return 0;

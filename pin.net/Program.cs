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

var rng = new Random();
Console.WriteLine(rng.Next(0, (int)Math.Pow(10, n)));
return 0;

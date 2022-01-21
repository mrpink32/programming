// See https://aka.ms/new-console-template for more information

string[] RouteData = File.ReadAllLines("D:/C# projects/AdventOfCode2021/AdventOfCode2021Day2/RouteData.txt");
#region part 1
int horizontalPosition = 0;
int depth = 0;
foreach (string direction in RouteData)
{
    string[] data = direction.Split(' ');
    if (data[0].Equals("forward"))
    {
        horizontalPosition += int.Parse(data[1]);
        //Console.WriteLine(direction);
        //Console.WriteLine(direction[direction.Length - 1]);
    }
    else if (data[0].Equals("down"))
    {
        depth += int.Parse(data[1]);
        //Console.WriteLine(direction);
        //Console.WriteLine(direction[direction.Length - 1]);
    }
    else if (data[0].Equals("up"))
    {
        depth -= int.Parse(data[1]);
        //Console.WriteLine(direction);
        //Console.WriteLine(direction[direction.Length - 1]);
    }
    //Console.WriteLine(depth);
}
int result = horizontalPosition * depth;
Console.WriteLine(result);
#endregion

#region part 2
horizontalPosition = 0;
depth = 0;
int aim = 0;
foreach (string direction in RouteData)
{
    string[] data = direction.Split(' ');
    if (data[0].Equals("forward"))
    {
        horizontalPosition += int.Parse(data[1]);
        depth += aim * int.Parse(data[1]);
    }
    else if (data[0].Equals("down"))
    {
        aim += int.Parse(data[1]);
    }
    else if (data[0].Equals("up"))
    {
        aim -= int.Parse(data[1]);
    }
}
result = horizontalPosition * depth;
Console.WriteLine(result);
#endregion
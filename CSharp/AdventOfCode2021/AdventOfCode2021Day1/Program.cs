string[] depthData = File.ReadAllLines("D:/C# projects/AdventOfCode2021/AdventOfCode2021Day1/DepthData.txt");
#region Part 1
int previousNumber = int.Parse(depthData[0]);
int increaseCount = 0;
for (int i = 1; i < depthData.Length - 1; i++)
{
    int currentNumber = int.Parse(depthData[i]);
    if (currentNumber > previousNumber)
    {
        increaseCount++;
    }
    previousNumber = currentNumber;
}
Console.WriteLine(increaseCount);
#endregion

#region Part2
previousNumber = int.MaxValue;
increaseCount = 0;
for (int i = 0; i < depthData.Length-2; i++)
{
    int currentNumber = int.Parse(depthData[i]) + int.Parse(depthData[i+1]) + int.Parse(depthData[i+2]);
    if (currentNumber > previousNumber)
    {
        increaseCount++;
    }
    previousNumber = currentNumber;
}
Console.WriteLine(increaseCount);
#endregion

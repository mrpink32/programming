// See https://aka.ms/new-console-template for more information
string[] diagnosticOutput = File.ReadAllLines("D:/C# projects/AdventOfCode2021/AdventOfCode2021Day3/Diagnostic.txt");
int[] result = new int[12];
for (int i = 0; i < diagnosticOutput[i].Length; i++)
{
    int[] nBit = new int[diagnosticOutput.Length];
    //List<int> nBit = new List<int>();
    for (int j = 0; j < nBit.Length; j++)
    {
        Console.WriteLine(diagnosticOutput[j][diagnosticOutput[j].Length - (1 + i)]);
        nBit[j] = diagnosticOutput[j][diagnosticOutput[j].Length - (1 + i)];
    }
    //foreach (string output in diagnosticOutput)
    //{
    //    Console.WriteLine(output[output.Length-(1+i)]);
    //    nBit.Append(output[output.Length - (1 + i)]);
    //    //nBit.Add(output[output.Length - (1 + i)]);
    //}
    foreach (var item in nBit)
    {
        Console.WriteLine($"grim: {item}");
    }
    result[i] = GetTypetal(nBit);
    Console.WriteLine($"{i+1}: finished");
}
foreach (var item in result)
{
    Console.WriteLine(item);
}


int GetTypetal(int[] data)
{
    int count0 = 0;
    int count1 = 0;
    foreach (int item in data)
    {
        if (item.Equals(0))
        {
            count0++;
        }
        else
        {
            count1++;
        }
    }
    if (count0 < count1) { return 1; } 
    else { return 0; }
}

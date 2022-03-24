using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace CurrencyConverter
{
    class Program
    {
        static string conversion;
        static string amount;
        static int value;
        static void Main(string[] args)
        {
            ConvertCurrency();
        }
        static void ConvertCurrency()
        {
            Console.WriteLine("Enter the desired currency conversion: \nExample: \"dkk to usd\"");
            conversion = Console.ReadLine();

            switch (conversion)
            {
                default:
                    Console.WriteLine("Sorry, that conversion hasn't been added yet");
                    Console.ReadLine();
                    ConvertCurrency();
                    break;
                case "dkk to usd":
                    Console.WriteLine("Please input the amount you want to convert:");
                    amount = Console.ReadLine();
                    value = amount.IndexOf("") + 1;
                    Console.WriteLine($"{value}");
                    Console.ReadLine();
                    break;

            }
        }
    }
}

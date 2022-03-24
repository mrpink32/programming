using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_11
{
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {

                Console.WriteLine("Skriv en tekst og se den bagfra");
                string input = Console.ReadLine();
                string bagfra = "";

                for (int i = input.Length - 1; i >= 0; i--)
                {
                    bagfra = bagfra + input[i];
                }

                Console.WriteLine(bagfra);
                Console.ReadLine();

            }



        }
    }
}

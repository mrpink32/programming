using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Array
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] games = new string[5];

            int sIndex = 0;
            int sLength = 0;


            for (int i = 0; i<5; i++ )
            {
                string input = Console.ReadLine();
                games[i] = input;

                if (sLength < input.Length) {
                    sIndex = i;
                    sLength = input.Length;
                }
            }
            
            Console.WriteLine(games[sIndex]+" var det længste ord og det havde en længde på "+sLength);
            Console.ReadLine();










        }
    }
}

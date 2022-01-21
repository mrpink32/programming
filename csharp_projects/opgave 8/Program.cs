using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_8_dictionary
{
    class Program
    {
        static void Main(string[] args)
        {

            Dictionary<string, float> nar = new Dictionary<string, float>();

            nar.Add("I am legend", 10);
            nar.Add("Overlord", 3);
            nar.Add("Captain marvel", 7);
            nar.Add("Ready player one", 7);

            while(true)
            {
                string input = Console.ReadLine();

                if(nar.ContainsKey(input))
                {
                    Console.WriteLine("Rating for filmen er på " + nar[input]);
                    Console.ReadLine();
                }
                else
                {
                    Console.WriteLine("filmen er ikke i databasen");
                    Console.ReadLine();
                }
                


            }

        } 
    }
}

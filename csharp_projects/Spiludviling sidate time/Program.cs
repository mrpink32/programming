using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Spiludviling_sidate_time
{
    class Program
    {
        static void Main(string[] args)
        {
            int antalTrue = 0;
            int antalFalse = 0;
            Dictionary<string, bool> niceDictionary = new Dictionary<string, bool>() { { "Pelle", false }, { "Sol", true }, { "Nicolas", true }, { "Albert", true }, { "Sebastian", false }, { "Alexander", true }, { "Anton", false }, { "Jasmin", false }, { "Mads Emil", true }, { "Mikkel", true }, { "Nikolaj", false } };
            Console.WriteLine("Der er " + niceDictionary.Count + " børn på listen.");
            foreach (KeyValuePair<string, bool> kvp in niceDictionary)
            {
                if (kvp.Value == true)
                {
                    antalTrue++;
                }
                if (kvp.Value == false)
                {
                    antalFalse++;
                }
            }

            Console.WriteLine("Der er " + antalTrue + " antal nice folk");
            Console.WriteLine("Der er " + antalFalse + " antal ikke nice folk");

            string firtstNicePerson = niceDictionary.Where(pair => pair.Value == true)
                    .Select(pair => pair.Key)
                    .First();
            Console.WriteLine("Den første nice person er... " + firtstNicePerson + "!!!!!");

            string lastIkkeNicePerson = niceDictionary.Where(pair => pair.Value == false)
                    .Select(pair => pair.Key)
                    .Last();
            Console.WriteLine("Den sidste ikke nice person er... " + lastIkkeNicePerson + "!!!!!:(");

            foreach (KeyValuePair<string, bool> makeTrue in niceDictionary)
            {
                
            }

            Console.ReadLine();
        }
    }
}

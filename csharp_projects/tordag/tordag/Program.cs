using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


namespace tordag
{
    class Program
    {
        static void Main(string[] args)
        {

            Console.WriteLine("*** Nævn et spil ***");
            List<string> games = new List<string>();

            games.Add("Borderlands2");
            games.Add("Terraria");
            games.Add("League of Legends");
            games.Add("Dead Cells");
            games.Add("PUBG");
            games.Add("No Man's Sky");


            while (true)
            {
                games.Add(Console.ReadLine());
                Console.WriteLine("den forrige var: " + games[games.Count - 2]);
            }
            Console.ReadLine();











            
        }
    }
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Doublelist
{
    class Program
    {
        static void Main(string[] args)
        {

            List<string> names = new List<string>(){"pelle", "sol", "mikkel"};
            List<string> rooms = new List<string>(){ "th114", "th112", "th313"};
      

            for(int i=0; i<names.Count; i++)
            {

                Console.WriteLine(names[i]+" bord på værelse "+rooms[i]);
                Console.ReadLine();

            }

          








        }
    }
}

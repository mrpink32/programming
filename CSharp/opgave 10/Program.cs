using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_10
{
    class Program
    {
        static void Main(string[] args)
        {
            List<float> fl = new List<float>();

            fl.Add(9.3f);
            fl.Add(8.4f);
            fl.Add(5.1f);
            fl.Add(1.6f);
            fl.Add(6.7f);
            fl.Add(7.2f);
            fl.Add(921.823f);
            fl.Add(6561.984f);
            fl.Add(95.954f);
            fl.Add(95.6f);
            



            Console.WriteLine("Der er "+fl.Count+" tal i listen. Det max nummer er "+fl.Max()+", det laveste er "+fl.Min()+" og gennemsnits tallet er "+fl.Average());
            Console.ReadLine();

        }
    }
}

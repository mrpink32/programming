using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_14
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Der er blevet udskrevet " + AebleNan() + " gange");
            Console.ReadLine();
        }
        static int AebleNan()
        {
            int times = 0;
            bool yes = true;
            Random random = new Random();

            List<string> strings = new List<string>();

            strings.Add("aeble");
            strings.Add("Banan");

            while(yes)
            {
                if(random.Next(0, 2) == 1)
                {
                    Console.WriteLine("æble");
                    strings.Add("æble");
                }
                else
                {
                    Console.WriteLine("banan");
                    strings.Add("banan");
                }
                times++;
                if ((strings[times - 1]== strings[times] && strings[times] == strings[times + 1]))
                {
                    yes = false;
                }
            }
         return times;  
        }
        
    }
}

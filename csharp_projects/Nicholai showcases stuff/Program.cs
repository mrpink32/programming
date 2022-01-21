using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Nicholai_showcases_stuff
{
    class Program
    {
        static int myInt;
        static void Main(string[] args)
        {
            myInt = 5;
            string svar = extraMethod(20);
            Console.WriteLine(svar);
            Console.ReadLine();
        }
        static string extraMethod(int input)
        {
            int ganget = myInt * input;
            string retur = "";
            if (ganget <= 100)
            {
                 retur = "Det var godt nok et lille tal det der";
            }
            else
            {
                 retur = "Det var godt nok et stort tal det der";
            }
            return retur;
        }
    }
}

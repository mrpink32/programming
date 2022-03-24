using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_12
{
    class Program
    {
        static void Main(string[] args)
        {
            while (true)
            {
                Console.WriteLine("Skriv den ene side længde på dit kvadrat");
                int input = Convert.ToInt32(Console.ReadLine());
                int kvadratet = Kvadrat(input);
                Console.WriteLine("Dit kvadrat har en omkreds på: " + kvadratet);

                Console.WriteLine("Skriv din cirkels radius");

            }
        }


        static int Kvadrat(int side1)
        {
            return side1 * 4;
        }

        static int CirkelAreal(int radius1)
        {
            return radius1 * radius1 * Math.PI;
        }










        
    }   

}

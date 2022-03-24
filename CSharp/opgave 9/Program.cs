using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_9
{
    class Program
    {
        static void Main(string[] args)
        {
            Random random = new Random();

            int d1 = random.Next(1, 7);
            int d2 = random.Next(1, 7);
            int d3 = random.Next(1, 7);
            int d4 = random.Next(1, 7);
            int d5 = random.Next(1, 7);
            int count = 1;

            Console.WriteLine(d1+" "+d2+" "+d3+" "+d4+" "+d5);
            

            while (!((d1==d2) && (d2==d3) && (d3==d4) && (d4==d5)))
            {
                 d1 = random.Next(1, 7);
                 d2 = random.Next(1, 7);
                 d3 = random.Next(1, 7);
                 d4 = random.Next(1, 7);
                 d5 = random.Next(1, 7);

                Console.WriteLine(d1 + " " + d2 + " " + d3 + " " + d4 + " " + d5);
                

                count++;

            }

            Console.WriteLine("Der blev slået Yatsy på " + count + " slag og det var fem gange " + d1);
            Console.ReadLine();
        }
    }
}

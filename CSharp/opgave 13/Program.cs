using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace opgave_13
{
    class Program
    {
        static void Main(string[] args)
        {
            while(true)
            {
                Console.WriteLine("Indtast et tal:");
                int ind = Convert.ToInt32(Console.ReadLine());
                if (isPrime(ind))
                {
                    Console.WriteLine(ind + " er et primtal");
                }
                else
                {
                    Console.WriteLine(ind + " er ikke et primtal");
                }
                Console.WriteLine();
            }

        }

        static bool isPrime(int t)
        {
            bool prime = true;
            for (int i = 2; i <= Math.Sqrt(t); i++)
            {
                Console.WriteLine(t + " modulus " + i + " er " + t % i);
                if (t % i == 0)
                {
                    prime = false;
                }
            }
            return prime;
        }


    }
}

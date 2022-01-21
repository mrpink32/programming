using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace loop_i_en_liste
{
    class Program
    {
        static void Main(string[] args)
        {

            Random random = new Random();
            int times = random.Next(10, 30);
            int lIndex = 0;
            int hIndex = 0;
            float sum = 0;

            List<float> lFloats = new List<float>();

            for (int i = 0; i < times; i++);
            {
                float f = (float)random.NextDouble() * random.Next(1, 20);
                lFloats.Add(f);
                Console.WriteLine(f);
            }

            sum = sum + lFloats[0];

            for(int i = 1; i < lFloats.Count; i++)
            {
                if (lFloats[lIndex] > lFloats[i])
                {
                    lIndex = i;
                }
                if (lFloats[lIndex] > lFloats[i])
                {
                    hIndex = i;
                }

                sum = sum + lFloats[i];
            }

            Console.WriteLine("Listen indeholder " + lFloats.Count);
            Console.WriteLine("Listens mindste værdi er på index " + lIndex + " som er " + lFloats[lIndex]);













        }
    }
}

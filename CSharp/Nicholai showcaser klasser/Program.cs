using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Nicholai_showcaser_klasser
{
    class Program
    {
        static void Main(string[] args)
        {
            Dna Pelle = new Dna();
            Dna Alex = new Dna();
            Pelle.number = 10;
            Console.WriteLine(Pelle.number);
            
            Console.ReadLine();
        }
    }
    public class Dna
    {
        public int number = 5;
    }
}

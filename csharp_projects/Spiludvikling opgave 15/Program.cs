using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Spiludvikling_opgave_15
{
    class Program
    {   
        static void Main(string[] args)
        {
        
            
         Game();
            

        }
        static void Game()
        {
            Console.WriteLine("Welcome to the game");
            Console.WriteLine("You have 10 tries to get the 4 letters right. The letters consist of the alpabet from A to F");
            Random random = new Random();
            List<string> letters = new List<string>();
            letters.Add("a");
            letters.Add("b");
            letters.Add("c");
            letters.Add("d");
            letters.Add("e");
            letters.Add("f");
            string l1 = letters[random.Next(0, 5)];
            string l2 = letters[random.Next(0, 5)];
            string l3 = letters[random.Next(0, 5)];
            string l4 = letters[random.Next(0, 5)];
            string randomWord = l1 + l2 + l3 + l4;
            for (int lives = 10; lives > 0; lives--)
            {
                if (lives <= 9)
                {
                    Console.WriteLine("Try again... You have " + lives + " tries left!");
                }
            string input = Console.ReadLine();
                if (input == randomWord)
                {
                    Console.WriteLine("Congrats");
                    Console.ReadLine();
                    Replay();
                }            
            }
            Console.WriteLine("You lose Boi");
            Console.ReadLine();
            Replay();
        }
        static void Replay()
        {
            Console.WriteLine("Do you want to play again?");
            string decision = Console.ReadLine();
            if (decision == "y" || decision == "ye" || decision == "yes")
            {
                Game();
            }
            else if (decision == "n" || decision == "no")
            {
                Console.WriteLine("Byebye");
                Console.ReadLine();
            }
            else
            {
                Console.WriteLine("That is not an available command");
                Replay();
            }
        }
    }
}

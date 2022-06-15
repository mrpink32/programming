using System;
using System.Collections.Generic;
using System.Text;
using Sys = Cosmos.System;

namespace CosmosKernel1
{
	public class Kernel : Sys.Kernel
	{


		protected override void BeforeRun()
		{
			Console.WriteLine("Cosmos booted successfully. Welcome to shittOS!");
			Console.WriteLine("Type \"help\" to see a list of available commands");
		}

		protected override void Run()
		{
			Console.Write("enter command> ");
			var input = Console.ReadLine().ToLower();
			switch (input)
			{
				case "help":
					HelpCommand();
					break;
				case "about":
					Console.WriteLine("Welcome to shittOS!\nThe shitty os made by someone who has no idea how to make an os!");
					break;
				case "clear":
					Console.Clear();
					break;
				case "shutdown":
					Sys.Power.Shutdown();
					break;
				default:
					Console.WriteLine("unrecognized command!");
					break;
			}
		}

		void HelpCommand()
		{
			Console.WriteLine("Commands:\nhelp\nabout\nclear\nshutdown");
		}

	}
}

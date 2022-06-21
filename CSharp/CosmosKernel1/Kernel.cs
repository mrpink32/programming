using System;
using System.Text;
using System.Drawing;
using System.Collections.Generic;
using Sys = Cosmos.System;
using Cosmos.System.Graphics;

namespace CosmosKernel1
{
	public class Kernel : Sys.Kernel
	{
		Canvas desktop;
		bool isGUIMode = false;
		Pen pen = new(Color.GreenYellow);

		protected override void BeforeRun()
		{
			Console.WriteLine("Cosmos booted successfully. Welcome to shittOS!");
			Console.WriteLine("Type \"help\" to see a list of available commands");
		}

		protected override void Run()
		{
			if (isGUIMode)
			{

				try
				{
					// todo calculate fps
					//if (_deltaT != RTC.Second)
					//{
					//	_fps = _frames;
					//	_frames = 0;
					//	_deltaT = RTC.Second;
					//}
					//_frames++;

					DrawDesktop();

					// draw fps
					//desktop.DrawString();

					//Console.ReadKey();
					//desktop.Mode = new Mode(800, 600, ColorDepth.ColorDepth32);
				}
				catch (Exception)
				{
					//mDebugger.Send("Exception occurred: " + e.Message);
					//mDebugger.Send(e.Message);
					Stop();
				}
			}
			else
			{
				Console.Write("> ");
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
						Stop();
						//Sys.Power.Shutdown();
						break;
					case "gui-mode":
						Console.Clear();
						desktop = FullScreenCanvas.GetFullScreenCanvas();
						desktop.Clear(Color.FromArgb(200, 150, 255));
						isGUIMode = true;
						break;
					default:
						Console.WriteLine("unrecognized command!");
						break;
				}
			}
		}
		void HelpCommand()
		{
			Console.WriteLine("Commands:\nhelp\nabout\nclear\ngui-mode\nshutdown");
		}

		void DrawDesktop()
		{
			
			desktop.DrawFilledRectangle(pen, 0, 0, 750, 100);

		}
	}
}

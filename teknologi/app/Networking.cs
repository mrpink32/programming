using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Net.Sockets;

namespace app
{
	internal static class Networking
	{
		public static string esp32address = "192.168.4.1";
		public static string otherAddress = "10.156.188.81";
		public static string homeAddress = "192.168.0.25";
		public static string localHost = "127.0.0.1";
		public static int port = 9000;

		public static TcpClient client;
		public static NetworkStream netStream;
		private static byte[] msg;

		public static float moistValue;
		public static float progress = 0f;
		public static float maxMoistValue = 4095;
		public static float minMoistValue = 800f;

		static void MainLoop()
		{
			while (client == null)
			{
				OpenConnection(esp32address);
				while (client.Connected)
				{
					try
					{
						moistValue = ReceiveFloat();
						progress = moistValue / maxMoistValue;
						Console.WriteLine($"{moistValue} {progress}");
					}
					catch (Exception ex)
					{
						Console.WriteLine(ex.ToString());
						client = null;
						break;
					}

				}
			}
		}

		public static void OpenConnection(string address)
		{
			while (true)
			{

				Console.WriteLine("Connecting to board");
				try
				{
					client = new TcpClient();
					client.Connect(address, port);
					netStream = client.GetStream();
					Console.WriteLine("Connected to board");
					break;
				}
				catch (Exception ex)
				{
					client = null;
					Console.WriteLine(ex.ToString());
				}
			}
		}

		public static float ReceiveFloat()
		{
			msg = new byte[client.ReceiveBufferSize];
			int byteRead = netStream.Read(msg, 0, msg.Length);
			return float.Parse(Encoding.ASCII.GetString(msg, 0, byteRead));
		}
	}
}

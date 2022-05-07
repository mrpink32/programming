using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net.Sockets;

namespace ConsoleApp2 // Note: actual namespace depends on the project name.
{
	internal class Program
	{
		public static string esp32address = "192.168.4.1";
		public static string otherAddress = "10.156.188.81";
		public static string home = "192.168.0.25";
		public static string localHost = "127.0.0.1";
		public static int port = 9000;

		public static TcpClient client;
		public static NetworkStream netStream;
		public static byte[] msg;

		public static float meterValue;
		public static float progress;
		public static int maxMeterValue = 4095;

		static void Main(string[] args)
		{
			while (client == null)
			{
				OpenConnection(otherAddress);
				while (client.Connected)
				{
					try
					{
						meterValue = ReceiveInt();
						progress = meterValue / maxMeterValue;
						Console.WriteLine($"{meterValue} {progress}");
					}
					catch (Exception ex)
					{
						client.Close();
						client = null;
						Console.WriteLine(ex.ToString());
					}
					
				}
			}
		}

		public static void OpenConnection(string address)
		{
			while (true)
			{
				try
				{
					Console.WriteLine("Connecting to board");
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

		public static int ReceiveInt()
		{
			byte[] msg = new byte[client.ReceiveBufferSize];
			int byteRead = netStream.Read(msg, 0, msg.Length);
			return int.Parse(Encoding.ASCII.GetString(msg, 0, byteRead));
		}
	}
}




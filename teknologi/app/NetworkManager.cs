using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net.Sockets;

namespace app
{
	internal static class NetworkManager
	{
        private static string ip = "10.156.188.93";
        private static string localHost = "127.0.0.1";
        private static int port = 9000;

        private static TcpClient client;
		private static NetworkStream netStream;

		public static void OpenConnection()
		{
            if (client != null)
            {
                Console.WriteLine("Connection already open...");
            }
            else
            {
                try
                {
                    client = new TcpClient();
                    client.Connect(ip, port);
                    netStream = client.GetStream();
                }
                catch (Exception ex)
                {
                    client = null;
                    Console.WriteLine($"Error: connection failed... msg: {ex}");
                }
            }
        }

	}
}

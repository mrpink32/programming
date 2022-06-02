using System.Net.Sockets;
using System.Text;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
TcpClient client = new();

while (!client.Connected)
{
	client.Connect("localhost", 9000);
	NetworkStream netStream = client.GetStream();
	while (client.Connected)
	{
		try
		{
			netStream.Write(Encoding.UTF8.GetBytes("Hello顔\n"));
			//byte[] packet = new byte[client.ReceiveBufferSize];
			//int bytesRead = netStream.Read(packet, 0, packet.Length);
			//string message = Encoding.UTF8.GetString(packet);
			//message = message.Replace("\n", "");
			//Console.WriteLine(message);
			Thread.Sleep(1000);
		}
		catch (Exception)
		{
			Console.WriteLine("grim");
			break;
		}
		
	}
}

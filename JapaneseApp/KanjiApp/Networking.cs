using System.Net.Sockets;
using System.Text;

namespace KanjiApp;

internal static class Networking
{
	public static string esp32address = "192.168.4.1";
	public static string otherAddress = "10.156.188.81";
	public static string homeAddress = "195.249.51.75";
	public static string realHomeAddress = "192.168.0.25";
	public static string localHost = "127.0.0.1";
	public static int port = 9000;

	public static TcpClient client = new();
	public static NetworkStream netStream;

	public static void OpenConnection(string address)
	{
		while (!client.Connected)
		{
			try
			{
				System.Diagnostics.Debug.WriteLine("Connecting to server");
				client.Connect(address, port);
				netStream = client.GetStream();
			}
			catch (Exception ex)
			{
				System.Diagnostics.Debug.WriteLine(ex.ToString());
			}	
		}
		System.Diagnostics.Debug.WriteLine("Connected to server");
	}

	public static void SendData(int message)
	{
		try
		{
			byte[] packet = Encoding.UTF8.GetBytes($"{message}\n");
			netStream.Write(packet, 0, packet.Length);
		}
		catch (Exception ex)
		{
			System.Diagnostics.Debug.WriteLine(ex.ToString());
		}
	}
	public static void SendData(string message)
	{
		try
		{
			byte[] packet = Encoding.UTF8.GetBytes($"{message}\n");
			netStream.Write(packet, 0, packet.Length);
		}
		catch (Exception ex)
		{
			System.Diagnostics.Debug.WriteLine(ex.ToString());
		}
	}
	
	public static string ReceiveData()
	{
		try
		{
			byte[] packet = new byte[client.ReceiveBufferSize];
			int bytesRead = netStream.Read(packet, 0, packet.Length);
			string message = Encoding.UTF8.GetString(packet, 0, bytesRead);
			return message.Replace("\n", "");
		}
		catch (Exception ex)
		{
			System.Diagnostics.Debug.WriteLine(ex.ToString());
			return "";
		}
	}

	public static string RetreiveData()
	{
		SendData("0:");
		return ReceiveData();
	}
}


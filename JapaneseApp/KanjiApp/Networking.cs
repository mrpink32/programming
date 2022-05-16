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

	public static TcpClient client;
	public static NetworkStream netStream;

	public static void OpenConnection(string address)
	{
		while (true)
		{
			Console.WriteLine("Connecting to server");
			try
			{
				client = new TcpClient();
				client.Connect(address, port);
				netStream = client.GetStream();
				Console.WriteLine("Connected to server");
				break;
			}
			catch (Exception ex)
			{
				client = null;
				Console.WriteLine(ex.ToString());
			}
		}
	}

	#region ReceivePackets
	public static byte ReceiveByte()
	{
        try
        {
            byte[] msg = new byte[client.ReceiveBufferSize];
            int byteRead = netStream.Read(msg, 0, msg.Length);
            return byte.Parse(Encoding.UTF8.GetString(msg, 0, byteRead));
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.ToString());
            throw;
        }
    }

	public static int ReceiveInt()
	{
        try
        {
            byte[] msg = new byte[client.ReceiveBufferSize];
            int byteRead = netStream.Read(msg, 0, msg.Length);
            return int.Parse(Encoding.UTF8.GetString(msg, 0, byteRead));
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.ToString());
            throw;
        }
    }

    public static float ReceiveFloat()
    {
		try
		{
            byte[] msg = new byte[client.ReceiveBufferSize];
            int byteRead = netStream.Read(msg, 0, msg.Length);
            return float.Parse(Encoding.UTF8.GetString(msg, 0, byteRead));
        }
		catch (Exception ex)
		{
            System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
    }

    public static string ReceiveString()
	{
		try
		{
            byte[] msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return Encoding.UTF8.GetString(msg, 0, bytesRead);
        }
		catch (Exception ex)
		{
            System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
	}
	#endregion

	#region SendPackets
	public static void SendData(byte data)
	{
		try
		{
			byte[] msg = BitConverter.GetBytes(data);
			netStream.Write(msg, 0, msg.Length);
		}
		catch (Exception ex)
		{
			//Console.WriteLine($"Error sending data to server via TCP: {ex}");
			System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
	}

	public static void SendData(int data)
	{
		try
		{
			if (client != null)
			{
				byte[] msg = BitConverter.GetBytes(data);
				netStream.Write(msg, 0, msg.Length);
			}
		}
		catch (Exception ex)
		{
			//Console.WriteLine($"Error sending data to server via TCP: {ex}");
			System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
	}

	public static void SendData(string data)
	{
		try
		{
			byte[] msg = Encoding.UTF8.GetBytes(data);
			netStream.Write(msg, 0, msg.Length);
		}
		catch (Exception ex)
		{
			//Console.WriteLine($"Error sending data to server via TCP: {ex}");
			System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
	}

	public static void SendData(byte[] data)
	{
		try
		{
			if (client != null)
			{
				SendData(data.Length);
				for (int i = 0; i < data.Length; i++)
				{
					//Console.WriteLine($"byte sent! i: {i+1}");
					SendData(data[i]);
				}
			}
		}
		catch (Exception _ex)
		{
			Console.WriteLine($"Error sending data to server via TCP: {_ex}");
		}
	}

	public static void SendData(string[] data)
	{
		try
		{
			if (client != null)
			{
				SendData(data.Length);
				for (int i = 0; i < data.Length; i++)
				{
					//Console.WriteLine($"byte sent! i: {i+1}");
					SendData(data[i]);
				}
			}
		}
		catch (Exception _ex)
		{
			Console.WriteLine($"Error sending data to server via TCP: {_ex}");
		}
	}

	public static void SendByte(byte data)
    {
        if (client == null)
        {
            Console.WriteLine("Error: connection is not open");
            return;
        }
        //msg = Encoding.Default.GetBytes($"{data}");
        //netStream.Write(msg, 0, msg.Length);
        netStream.WriteByte(data);
    }

    public static void SendInt(int data)
    {
        if (client == null)
        {
            Console.WriteLine("Error: connection is not open");
            return;
        }
        byte[] msg = Encoding.UTF8.GetBytes($"{data}");
        netStream.Write(msg, 0, msg.Length);
        //netStream.WriteByte(((byte)data));
    }

    public static void SendString(string data)
    {
		try
		{
			byte[] msg = Encoding.UTF8.GetBytes($"{data}");
			netStream.Write(msg, 0, msg.Length);
		}
		catch (Exception ex)
		{
			System.Diagnostics.Debug.WriteLine(ex.ToString());
			throw;
		}
    }

	public static void SendByteArray(byte[] data)
    {
        SendInt(data.Length);
        netStream.Write(data, 0, data.Length);
    }


    public static void SendStringArray(string[] data)
    {
        SendInt(data.Length);
        for (int i = 0; i < data.Length; i++)
        {
            SendString(data[i]);
            string returnMsg = ReceiveString();
            if (!returnMsg.Equals("item received"))
            {
                Console.WriteLine("client didn't return with answer");
                break;
            }
        }
    }
    #endregion
}

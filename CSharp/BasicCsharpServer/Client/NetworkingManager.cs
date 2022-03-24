using System;

public class NetworkingManager
{
	public NetworkingManager()
	{
	}

    #region ReceiveData
    private static byte ReceiveByte()
    {
        //msg = new byte[client.ReceiveBufferSize];
        //int bytesRead = netStream.Read(msg, 0, msg.Length);
        //return byte.Parse(Encoding.Default.GetString(msg, 0, bytesRead));
        int data = netStream.ReadByte();
        return ((byte)data);
    }

    private static int ReceiveInt()
    {
        msg = new byte[client.ReceiveBufferSize];
        int bytesRead = netStream.Read(msg, 0, msg.Length);
        return int.Parse(Encoding.UTF32.GetString(msg, 0, bytesRead));
    }

    private static string ReceiveString()
    {
        msg = new byte[client.ReceiveBufferSize];
        int bytesRead = netStream.Read(msg, 0, msg.Length);
        return Encoding.UTF32.GetString(msg, 0, bytesRead);
    }

    private static byte[] ReceiveByteArray()
    {
        int arrayLength = ReceiveInt();
        byte[] data = new byte[arrayLength];
        for (int i = 0; i < arrayLength; i++)
        {
            data[i] = ReceiveByte();
        }
        return data;
    }

    private static MemoryStream ReceiveMusic()
    {
        int arrayLength = ReceiveInt();
        byte[] data = new byte[arrayLength];
        for (int i = 0; i < arrayLength; i++)
        {
            data[i] = ReceiveByte();
        }
        return new MemoryStream(data);
    }


    private static string[] ReceiveStringArray()
    {
        int arrayLength = ReceiveInt();
        string[] data = new string[arrayLength];
        for (int i = 0; i < arrayLength; i++)
        {
            data[i] = ReceiveString();
            SendString("item received");
        }
        return data;
    }
    #endregion

    #region SendData
    /*private static void SendData(byte data)
    {
        try
        {
            if (client != null)
            {
                msg = BitConverter.GetBytes(data);
                netStream.Write(msg, 0, msg.Length);
            }
        }
        catch (Exception _ex)
        {
            Console.WriteLine($"Error sending data to server via TCP: {_ex}");
        }
    }*/

    private static void SendByte(byte data)
    {
        if (client == null)
        {
            Console.WriteLine("Error: connection is not open");
            return;
        }
        //msg = Encoding.Unicode.GetBytes($"{data}");
        netStream.WriteByte(data);
    }

    private static void SendInt(int data)
    {
        if (client == null)
        {
            Console.WriteLine("Error: connection is not open");
            return;
        }
        //msg = Encoding.Default.GetBytes($"{data}");
        netStream.WriteByte(((byte)data));
    }


    private static void SendString(string data)
    {
        if (client == null)
        {
            Console.WriteLine("Error: connection is not open");
            return;
        }
        msg = Encoding.UTF32.GetBytes(data);
        netStream.Write(msg, 0, msg.Length);
    }
    #endregion
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;
using System.IO;

namespace FreesicClient
{
    public class NetworkManager
    {
        private readonly string ip;
        private readonly int port;
        public List<string> outputText = new();
        public string[] commands;
        public string[] songList;

        private TcpClient client;
        private NetworkStream netStream;

        public NetworkManager()
        {
            //ip = "195.249.51.75";
            ip = "127.0.0.1";
            port = 9000;
        }


        public void OpenConnection()
        {
            if (client != null) { outputText.Add("Connection already open..."); }
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
                    outputText.Add($"Error: connection failed... msg: {ex}");
                }
            }
        }


        public void CloseConnection()
        {
            if (client == null)
            {
                return;
            }

            try
            {
                netStream.Close();
                client.Close();
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Sheeeeeeesh: {ex}");
            }
            finally
            {
                client = null;
            }
        }


        #region ReceiveData
        private byte ReceiveByte()
        {
            int data = netStream.ReadByte();
            return ((byte)data);
        }
        private int ReceiveInt()
        {
            byte[] msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return int.Parse(Encoding.UTF32.GetString(msg, 0, bytesRead));
        }
        public string ReceiveString()
        {
            byte[] msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return Encoding.UTF32.GetString(msg, 0, bytesRead);
        }
        public byte[] ReceiveByteArray()
        {
            int arrayLength = ReceiveInt();
            byte[] data = new byte[arrayLength];
            for (int i = 0; i < arrayLength; i++)
            {
                data[i] = ReceiveByte();
            }
            return data;
        }
        public MemoryStream ReceiveMusic()
        {
            int arrayLength = ReceiveInt();
            byte[] data = new byte[arrayLength];
            for (int i = 0; i < arrayLength; i++)
            {
                data[i] = ReceiveByte();
            }
            return new MemoryStream(data);
        }
        public string[] ReceiveStringArray()
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
        public void SendByte(byte data)
        {
            try { netStream.WriteByte(data); }
            catch (Exception ex) { outputText.Add($"{ex}"); }
        }
        public void SendInt(int data)
        {
            byte[] msg = Encoding.UTF32.GetBytes(data.ToString());
            try { netStream.Write(msg, 0, msg.Length); }
            catch (Exception ex) { outputText.Add($"{ex}"); }
        }
        public void SendString(string data)
        {
            byte[] msg = Encoding.UTF32.GetBytes(data);
            try { netStream.Write(msg, 0, msg.Length); }
            catch (Exception ex) { outputText.Add($"{ex}"); }
        }
        #endregion

    }
}

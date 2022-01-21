using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Net;  
using System.Net.Sockets;
using System.IO;

namespace Server
{
    class Program
    {
        private static TcpListener server;
        private static readonly int port = 9000;

        private static TcpClient client;
        private static NetworkStream netStream;
        private static byte[] msg;

        private static readonly string[] commands = {"help", "music", "disconnect"};
        private static readonly string path = "D:/C# projects/BasicCsharpServer/Server/music/";
        private static string[] songs;

        private static DateTime startTime;
        private static DateTime endTime;

        static void Main(string[] args)
        {
            Console.Title = "Freesic Server";
            songs = new string[Directory.GetFiles(path).Length];
            for (int i = 0; i < Directory.GetFiles(path).Length; i++)
            {
                songs[i] = Directory.GetFiles(path)[i].ToString();
                songs[i] = songs[i].Replace(path, "").Replace(".mp3", "");
            }
            server = new TcpListener(IPAddress.Any, port);
            Console.WriteLine("Starting server....");
            server.Start();
            Console.WriteLine($"Server opened on port: {port}");
            while (true)
             {
                client = server.AcceptTcpClient();
                netStream = client.GetStream();
                Console.WriteLine($"Incomming connection from {client.Client.RemoteEndPoint}...");
                SendString("Welcome to the server!");
                try
                {
                    if (ReceiveString().Equals("connection check"))
                    {
                        Console.WriteLine("Client has successfully connected to the server!");
                        SendStringArray(commands);
                        SendStringArray(songs);
                    }
                }
                catch (Exception)
                {
                    Console.WriteLine("Connection lost");
                }
                while (client.Connected)
                 {
                    try
                    {
                        ReceiveCommand();
                    }
                    catch (Exception ex)
                    {
                        switch (ex)
                        {
                            case FormatException:
                                continue;
                            case IOException:
                                Console.WriteLine("Connection lost");
                                break;
                        }
                    }
                 }
             }
        }

        private static void ReceiveCommand()
        {
            string command = ReceiveString();
            if (command.Equals(commands[1]))
            {
                string songName = ReceiveString();
                byte[] audio = File.ReadAllBytes($"{path}{songName}.mp3");
                Console.WriteLine($"Upload started at: {startTime = DateTime.Now:HH:mm:ss}");
                SendByteArray(audio);
                Console.WriteLine($"Upload finished at: {endTime = DateTime.Now:HH:mm:ss}");
                Console.WriteLine($"Upload took: \n{endTime - startTime:mm}: minutes and \n{endTime - startTime:ss}: seconds");
                //ReceiveString();
            }
            else if (command.Equals(commands[2]))
            {
                netStream.Close();
                client.Close();
                Console.WriteLine("Client has disconnected");
            }
            else { Console.WriteLine($"command not recognized: {command}"); }
        }

        #region ReceivePackets
        private static byte ReceiveByte()
        {
            /*msg = new byte[client.ReceiveBufferSize];
            int bytesRead = stream.Read(msg, 0, msg.Length);
            return int.Parse(Encoding.Default.GetString(msg, 0, bytesRead));*/
            int data = netStream.ReadByte();
            return ((byte)data);
        }

        private static int ReceiveInt()
        {
            /*msg = new byte[client.ReceiveBufferSize];
            int bytesRead = stream.Read(msg, 0, msg.Length);
            return int.Parse(Encoding.Default.GetString(msg, 0, bytesRead));*/
            return netStream.ReadByte();
        }


        private static string ReceiveString()
        {
            msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return Encoding.UTF32.GetString(msg, 0, bytesRead);
        }
        #endregion

        #region SendPackets
       /* private static void SendData(byte data)
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
        }

        private static void SendData(int data)
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
        }

        private static void SendData(string data)
        {
            try
            {
                if (client != null)
                {
                    msg = Encoding.Default.GetBytes(data);
                    netStream.Write(msg, 0, msg.Length);
                }
            }
            catch (Exception _ex)
            {
                Console.WriteLine($"Error sending data to server via TCP: {_ex}");
            }
        }

        private static void SendData(byte[] data)
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

        private static void SendData(string[] data)
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
        }*/

        private static void SendByte(byte data)
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


        private static void SendInt(int data)
        {
            if (client == null)
            {
                Console.WriteLine("Error: connection is not open");
                return;
            }
            msg = Encoding.UTF32.GetBytes($"{data}");
            netStream.Write(msg, 0, msg.Length);
            //netStream.WriteByte(((byte)data));
        }


        private static void SendString(string data)
        {
            if (client == null)
            {
                Console.WriteLine("Error: connection is not open");
                return;
            }
            msg = Encoding.UTF32.GetBytes($"{data}");
            netStream.Write(msg, 0, msg.Length);
        }


        private static void SendByteArray(byte[] data)
        {
            //Old method to send a byte cluster
            //SendInt(data.Length);
            //for (int i = 0; i < data.Length; i++)
            //{
            //    SendByte(data[i]);
            //    byte returnMsg = ReceiveByte();
            //    if (!returnMsg.Equals(1))
            //    {
            //        Console.WriteLine("client didn't return with answer");
            //        break;
            //    }
            //}
            //new method to send a byte cluster
            //SendInt(data.Length);
            //for (int i = 0; i < data.Length; i++)
            //{
            //    //Console.WriteLine($"byte sent! i: {i+1}");
            //    SendByte(data[i]);
            //}
            //faster version of the new method to send a byte cluster
            SendInt(data.Length);
            netStream.Write(data, 0, data.Length);
        }


        private static void SendStringArray(string[] data)
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
            //SendInt(data.Length);
            //for (int i = 0; i < data.Length; i++)
            //{
            //    //Console.WriteLine($"byte sent! i: {i+1}");
            //    SendString(data[i]);
            //}
        }
        #endregion

    }
}

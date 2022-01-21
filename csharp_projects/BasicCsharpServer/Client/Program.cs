using System;
using System.Numerics;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Net;
using System.Net.Sockets;
using System.IO;
using System.IO.Compression;
using System.Threading;


namespace Client
{
    class Program
    {
        private static string ip = "195.249.51.75";
        private static string localHost = "127.0.0.1";
        private static int port = 9000;
        
        private static TcpClient client;
        private static NetworkStream netStream;
        private static byte[] msg;

        private static MemoryStream audio;
        private static string[] commands;
        private static string[] songs;
        private static Queue<byte[]> testQueue;

        private static DateTime startTime;
        private static DateTime endTime;

        //private static Thread downloadThread = new Thread(Download);


        static void Main(string[] args)
        {
            Console.Title = "TCP Client";
            OpenConnection();
            Console.WriteLine(ReceiveString());
            SendString("connection check");
            commands = ReceiveStringArray();
            songs = ReceiveStringArray();
            Console.WriteLine("type \"help\" to view a list of available commands");
            while (true)
             {
                string command = Console.ReadLine();
                if (command.Equals(commands[2])) { SendString(command); CloseConnection(); break; }
                else if (command.Equals(commands[0]))
                {
                    Console.WriteLine("commands:");
                    for (int i = 0; i < commands.Length; i++)
                    {
                        Console.WriteLine($"{i + 1}: {commands[i]}");
                    }
                }
                else if (command.Equals(commands[1]))
                {
                    SendString(command);
                    foreach (string item in songs)
                    {
                        Console.WriteLine($"{item}");
                    }
                    SendString($"{Console.ReadLine()}");
                    Console.WriteLine($"Download started at: {startTime = DateTime.Now:HH:mm:ss}");
                    //audio = ReceiveMusic();
                    //downloadThread.Start();
                    //Download();
                    MusicPlayer.SetupBufferingSource();
                    while (true)
                    {
                        int arrayLength = ReceiveInt();
                        byte[] msg = new byte[client.ReceiveBufferSize];
                        for (int i = 0; i < msg.Length; i++)
                        {
                            msg[i] = ReceiveByte();
                        }
                        MusicPlayer.src.Write(msg, 0, msg.Length);
                    }
                    Console.WriteLine($"Download finished at: {endTime = DateTime.Now:HH:mm:ss}");
                    Console.WriteLine($"Download took: \n{endTime - startTime:mm}: minutes and \n{endTime - startTime:ss}: seconds");
                    //Console.WriteLine($"{MusicPlayer._q.Count}");
                    //CreateAudioFile.CreateMP3(audio, $"{arrayLength}");
                    //Console.WriteLine("file created");
                    //MusicPlayer.PlayMusic(File.OpenRead($"D:/Github/BasicCsharpServer/Client/Temp/{arrayLength}.mp3"));
                    //MusicPlayer.PlayMusic(audio);
                    //MusicPlayer.PlayBufferedMusic();
                }
                else if (command == "stop")
                {
                    MusicPlayer.StopMusic();
                }
                else { SendString(command); }
             }
        }


        private static void OpenConnection()
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
                    client.Connect(localHost, port);
                    netStream = client.GetStream();
                }
                catch (Exception ex)
                {
                    client = null;
                    Console.WriteLine($"Error: connection failed... msg: {ex}");
                }
            }
        }


        private static void CloseConnection()
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

        private static void Download()
        {
            /*byte[] data = ReceiveByteArray();
            MusicPlayer._q.Enqueue(data);
            Console.WriteLine("Added array to buffer");
            while (netStream.DataAvailable)
            {
                byte[] data = new byte[client.ReceiveBufferSize];
                for (int i = 0; i < data.Length; i++)
                {
                    data[i] = ReceiveByte();
                }
                MusicPlayer._q.Enqueue(data);
                Console.WriteLine("Added array to buffer");
                MusicPlayer.UpdateBufferedSource();
            }*/

        }
    }
}

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Text;

using Xamarin.Forms;

namespace FreesicXam.Views
{
    public class Music : ContentPage
    {
        private string ip = "195.249.51.75";
        private string localHost = "127.0.0.1";
        private int port = 9000;

        private TcpClient client;
        private NetworkStream netStream;
        private byte[] msg;

        private MemoryStream audio;
        private string[] commands;
        private string[] songs;

        private DateTime startTime;
        private DateTime endTime;


        private ListView listView;


        private readonly string[] testArray = { "test", "test", "test", "test", "test", "test", "test", "test", "test", "test" };
        private readonly string path = "D:/GitHub/BasicCsharpServer/Server/music/";
        
        public Music()
        {
            OpenConnection();
            ReceiveString();
            SendString("connection check");
            commands = ReceiveStringArray();
            songs = ReceiveStringArray();



            ListView listView = new ListView()
            {
                ItemsSource = songs
                
            };
            listView.ItemSelected += ListView_ItemSelected;

            // Contruct the page.
            Title = "MusicPlayer";
            BackgroundColor = Color.FromHex("ff64ff");
            Content = new StackLayout
            {
                Children = {

                    listView
                }
            };
            

            void ListView_ItemSelected(object sender, SelectedItemChangedEventArgs e)
            {
                listView = sender as ListView;
                if (listView != null)
                {
                    Title = $"{listView.SelectedItem}";
                }
            }

        }


        #region Networking
        private void OpenConnection()
        {
            if (client != null)
            {
                //("Connection already open...");
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
                    //PopulateTextBlock($"Error: connection failed... msg: {ex}");
                }
            }
        }
        private void CloseConnection()
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
        private byte ReceiveByte()
        {
            int data = netStream.ReadByte();
            return ((byte)data);
        }
        private int ReceiveInt()
        {
            msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return int.Parse(Encoding.UTF32.GetString(msg, 0, bytesRead));
        }
        private string ReceiveString()
        {
            msg = new byte[client.ReceiveBufferSize];
            int bytesRead = netStream.Read(msg, 0, msg.Length);
            return Encoding.UTF32.GetString(msg, 0, bytesRead);
        }
        private byte[] ReceiveByteArray()
        {
            int arrayLength = ReceiveInt();
            byte[] data = new byte[arrayLength];
            for (int i = 0; i < arrayLength; i++)
            {
                data[i] = ReceiveByte();
            }
            return data;
        }
        private MemoryStream ReceiveMusic()
        {
            int arrayLength = ReceiveInt();
            byte[] data = new byte[arrayLength];
            for (int i = 0; i < arrayLength; i++)
            {
                data[i] = ReceiveByte();
            }
            return new MemoryStream(data);
        }
        private string[] ReceiveStringArray()
        {
            int arrayLength = ReceiveInt();
            string[] data = new string[arrayLength];
            //Console.WriteLine("commands:");
            for (int i = 0; i < arrayLength; i++)
            {
                data[i] = ReceiveString();
                //Console.WriteLine($"{i + 1}: {data[i]}");
                SendString("item received");
            }
            return data;
        }
        private void SendByte(byte data)
        {
            if (client == null)
            {
                Console.WriteLine("Error: connection is not open");
                return;
            }
            //msg = Encoding.Unicode.GetBytes($"{data}");
            netStream.WriteByte(data);
        }
        private void SendInt(int data)
        {
            if (client == null)
            {
                Console.WriteLine("Error: connection is not open");
                return;
            }
            //msg = Encoding.Default.GetBytes($"{data}");
            netStream.WriteByte(((byte)data));
        }
        private void SendString(string data)
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
}
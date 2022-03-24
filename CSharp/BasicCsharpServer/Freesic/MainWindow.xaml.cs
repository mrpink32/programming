using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Media;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Net.Sockets;
using System.IO;

namespace Freesic
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
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



        private readonly string[] testArray = { "test", "test", "test", "test", "test", "test", "test", "test", "test", "test" };
        private readonly string path = "D:/GitHub/BasicCsharpServer/Server/music/";

        public MainWindow()
        {
            OpenConnection();
            InitializeComponent();
            PopulateTextBlock(ReceiveString());
            SendString("connection check");
            commands = ReceiveStringArray();
            songs = ReceiveStringArray();
            PopulateBoxList(songs);
            PopulateTextBlock("type \"help\" to view a list of available commands");
            //Stream stream = File.OpenRead($"{path}KING Gawr Gura x Calliope Mori (Cover).mp3");
            //MusicPlayer.PlayMusic(stream);
        }

        private void PopulateBoxList(string[] data)
        {
            for (int i = 0; i < data.Length; i++)
            {
                testBox.Items.Add($"{data[i]}");
            }
        }

        private void PopulateTextBlock(string data)
        {
            outputWindow.Items.Add($"{data}");
        }

        private void ReceiveInput(object sender, KeyEventArgs keyEvent)
        {
            if (keyEvent.Key.Equals(Key.Enter))
            {
                string input = inputBox.Text;
                inputBox.Clear();
                PopulateTextBlock(input);
                HandleCommands(input);
            }
        }

        private void HandleCommands(string command)
        {
            if (command.Equals(commands[2])) { SendString(command); CloseConnection(); }
            else if (command.Equals(commands[0]))
            {
                //Console.WriteLine("commands:");
                PopulateTextBlock("commands:");
                for (int i = 0; i < commands.Length; i++)
                {
                    //Console.WriteLine($"{i + 1}: {commands[i]}");
                    PopulateTextBlock($"{i + 1}: {commands[i]}");
                }
            }
            else if (command.Equals(commands[1]))
            {
                SendString(command);
                foreach (string item in songs)
                {
                    //Console.WriteLine($"{item}");
                    PopulateTextBlock($"{item}");
                }
                SendString($"{Console.ReadLine()}");
                //Console.WriteLine($"Download started at: {startTime = DateTime.Now:HH:mm:ss}");
                PopulateTextBlock($"Download started at: {startTime = DateTime.Now:HH:mm:ss}");
                audio = ReceiveMusic();
                //downloadThread.Start();
                //Download();
                Console.WriteLine($"Download finished at: {endTime = DateTime.Now:HH:mm:ss}");
                Console.WriteLine($"Download took: \n{endTime - startTime:mm}: minutes and \n{endTime - startTime:ss}: seconds");
                //Console.WriteLine($"{MusicPlayer._q.Count}");
                //CreateAudioFile.CreateMP3(audio, $"{arrayLength}");
                //Console.WriteLine("file created");
                //MusicPlayer.PlayMusic(File.OpenRead($"D:/Github/BasicCsharpServer/Client/Temp/{arrayLength}.mp3"));
                MusicPlayer.PlayMusic(audio);
                //MusicPlayer.PlayBufferedMusic();
            }
            else if (command == "stop")
            {
                MusicPlayer.StopMusic();
            }
            else { SendString(command); }
        }

        private void TestBox_MouseDoubleClick(object sender, MouseButtonEventArgs e)
        {
            testBox = sender as ListBox;
            if (testBox != null)
            {
                SendString(commands[1]);
                string songName = testBox.SelectedItem.ToString();
                SendString($"{songName}");
                PopulateTextBlock($"{songName}");
                PopulateTextBlock("Click!");
                MusicPlayer.PlayMusic(ReceiveMusic());
            }
        }


        #region Networking
        private void OpenConnection()
        {
            if (client != null)
            {
                PopulateTextBlock("Connection already open...");
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
                    PopulateTextBlock($"Error: connection failed... msg: {ex}");
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

        private void volumeSlider_Drop(object sender, DragEventArgs e)
        {
            MusicPlayer.ChangeVolume(volumeSlider.Value);
        }
    }
}

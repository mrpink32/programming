using System.Net.Sockets;
using System.Net;
using System.Text;

string LOCALHOST = "localhost";
int PORT = 9000;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello World!");

// TcpClient client = new();
// TcpListener server = new();
IPAddress address = IPAddress.Parse("127.0.0.1");
IPEndPoint server = new(address, PORT);
Socket client = new(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
client.Connect(server);
while (true)
{
    string message = Console.ReadLine();
    byte[] packet = Encoding.UTF8.GetBytes(message);
    client.Send(packet);

    byte[] test1 = new byte[client.ReceiveBufferSize];
    int bytesRead = client.Receive(test1);
    string test2 = Encoding.UTF8.GetString(test1, 0, bytesRead);
    Console.WriteLine(test2);
}



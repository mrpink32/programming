using System.Net.Sockets;
using System.Text;

string LOCALHOST = "localhost";
int PORT = 9000;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello World!");

// TcpClient client = new();
Socket client = new(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
client.Connect(LOCALHOST, PORT);
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



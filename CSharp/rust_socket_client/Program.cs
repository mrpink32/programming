using System.Net.Sockets;
using System.Net;
using System.Text;

string LOCALHOST = "localhost";
int PORT = 9000;

IPAddress address = IPAddress.Parse("127.0.0.1");
IPEndPoint server = new(address, PORT);
TcpClient client = new();
// Socket client = new(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
client.Connect(LOCALHOST, PORT);
NetworkStream netStream = client.GetStream();
while (true)
{
    // string message = $"{Console.ReadLine()}\n";
    string message = $"test\n";
    byte[] packet = Encoding.UTF8.GetBytes(message);
    netStream.Write(packet, 0, packet.Length);

    byte[] test1 = new byte[client.ReceiveBufferSize];
    int bytesRead = netStream.Read(test1);
    string test2 = Encoding.UTF8.GetString(test1, 0, bytesRead);
    Console.Write(test2);
}



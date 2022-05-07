using System.Net.Sockets;
namespace app;

public partial class App : Application
{
    public App()
	{
		InitializeComponent();
        MainPage = new HomePage();
	}
}

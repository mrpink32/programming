using Microsoft.Maui;
using Microsoft.Maui.Controls;
using Microsoft.Maui.Controls.PlatformConfiguration.WindowsSpecific;
using Application = Microsoft.Maui.Controls.Application;

namespace FreesicClient
{
	public partial class App : Application
	{
		public App()
		{
			NetworkManager nw = new();
			nw.OpenConnection();
			InitializeComponent();
			MainPage = new MainPage(nw);
		}
	}
}

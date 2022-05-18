namespace KanjiApp;

public partial class App : Application
{
	public App()
	{
		/*Resources = new ResourceDictionary { 
			Source = new Uri("Resources/Styles.xaml")
		};*/
		//InitializeComponent();
		Networking.OpenConnection(Networking.realHomeAddress);
		MainPage = new MainPage();
	}
}

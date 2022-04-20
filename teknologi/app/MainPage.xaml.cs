namespace app;

public partial class MainPage : ContentPage
{
	int count = 0;

	public MainPage()
	{

		InitializeComponent();
	}

	private void OnCounterClicked(object sender, EventArgs e)
	{
		count++;
		NameLabel.Text = "Plant name";
		CounterLabel.Text = $"Current count: {count}";

		SemanticScreenReader.Announce(CounterLabel.Text);
	}
}


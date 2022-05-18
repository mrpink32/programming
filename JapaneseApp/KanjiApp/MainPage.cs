namespace KanjiApp;

public class MainPage : ContentPage
{
	public MainPage()
	{
		int count = 0;

		ScrollView scrollView = new();
		VerticalStackLayout verticalStackLayout = new()
		{
			Spacing = 25,
			Padding = 30
		};
		Label header = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			FontSize = 35,
			Text = "Plant overview:"
		};
		Label countLabel = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			FontSize = 35,
			Text = $"Current count: {count}"
		};
		Button buttonTest = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			Text = "Press me"
		};

		Content = scrollView;
		scrollView.Content = verticalStackLayout;
		verticalStackLayout.Children.Add(header);
		verticalStackLayout.Children.Add(countLabel);
		verticalStackLayout.Children.Add(buttonTest);
		buttonTest.Clicked += (sender, args) =>
		{
			count++;
			countLabel.Text = $"Current count: {count}";
			//Networking.SendString("水,すい,みず,Water/n");
			//Networking.SendData("stuff 1/n");
			Networking.SendData(count);
			//Networking.SendData("stuff 1" + "/n");
			System.Diagnostics.Debug.WriteLine("grim");
		};
	}
}
namespace app;

public class NewPage1 : ContentPage
{
	public NewPage1()
	{
		List<string> plants = new();
		if (plants.Count < 1)
		{
			plants.Add("Press the button below to add your first plant!");
		}

		Grid mainView = new()
		{
			VerticalOptions = LayoutOptions.Fill,
			HorizontalOptions = LayoutOptions.Fill,
		};
		for (int i = 0; i < 5; i++)
		{
			mainView.RowDefinitions.Add(new RowDefinition());
			mainView.ColumnDefinitions.Add(new ColumnDefinition());
		}
		Label header = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			Text = "Plants overview:",
			FontSize = 50
		};
		Button button = new()
		{
			HorizontalOptions= LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			Text = "Add new plant"
		};
		ListView listView = new()
		{
			ItemsSource = plants,
			HorizontalOptions= LayoutOptions.Center
			//VerticalOptions = LayoutOptions.Center,
		};

		Frame frame = new()
		{

		};

		mainView.SetRow(header, 0);
		mainView.SetColumnSpan(header, 5);
		mainView.SetRow(listView, 2);
		mainView.SetRowSpan(listView, plants.Count);
		mainView.SetColumnSpan(listView, 5);
		mainView.SetRow(button, plants.Count + 2);
		mainView.SetColumnSpan(button, 5);

		mainView.Children.Add(header);
		mainView.Children.Add(listView);
		mainView.Children.Add(button);
		Content = mainView;
	}
}
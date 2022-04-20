namespace app;

public class NewPage1 : ContentPage
{
	public NewPage1()
	{
		Label header = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			Text = "Plants overview:",
			FontSize = 50
		};
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


		mainView.SetRow(header, 0);
		mainView.SetColumn(header, 2);



		mainView.Children.Add(header);
		Content = mainView;
	}
}
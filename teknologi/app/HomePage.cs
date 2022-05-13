using System.Net.Sockets;
using System.Text;
using System.Threading;

namespace app;

public class HomePage : ContentPage
{
	List<PlantCard> plantCards = new();
	ScrollView mainView = new();
	VerticalStackLayout mainLayout = new()
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
	Button addPlantCardButton = new()
	{
		HorizontalOptions = LayoutOptions.Center,
		VerticalOptions = LayoutOptions.Center,
		Text = "Add new plant"
	};

	public HomePage()
	{
		Networking.OpenConnection(Networking.esp32address);
		Content = mainView;
		mainView.Content = mainLayout;
		addPlantCardButton.Clicked += addPlantCard;
		mainLayout.Children.Add(header);
		mainLayout.Children.Add(addPlantCardButton);
	}

	private void addPlantCard(object sender, EventArgs args)
	{
		int childCount = mainLayout.Children.Count;
		//PlantCard plantCard = new();
		//plantCards.Add(plantCard);
		mainLayout.Children.Insert(childCount - 1, new PlantCard());
	}
}
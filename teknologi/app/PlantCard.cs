using System.Net.Sockets;
using System.Text; 

namespace app;

public class PlantCard : ContentView
{
	public static void NetworkLoop()
	{
		while (Networking.client.Connected)
		{
			try
			{
				Networking.moistValue = Networking.ReceiveFloat();
				Networking.progress = Networking.moistValue / Networking.maxMoistValue;
				//System.Diagnostics.Debug.WriteLine(Networking.moistValue);
				//System.Diagnostics.Debug.WriteLine(Networking.progress);
				//System.Diagnostics.Debug.WriteLine(Networking.moistValue / Networking.maxMoistValue);
				//System.Diagnostics.Debug.WriteLine((Networking.moistValue / Networking.maxMoistValue).GetType());
				//System.Diagnostics.Debug.WriteLine(Networking.maxMoistValue);
				//System.Diagnostics.Debug.WriteLine(Networking.progress.GetType());
			}
			catch (Exception ex)
			{
				System.Diagnostics.Debug.WriteLine(ex.ToString());
			}
			Thread.Sleep(5000);
		}
	}
	Frame mainFrame = new()
	{
		BackgroundColor = Color.FromRgb(0, 0, 0)
	};
	VerticalStackLayout mainLayout = new();
	Label plantNameLabel = new()
	{
		HorizontalOptions = LayoutOptions.Center,
		VerticalOptions = LayoutOptions.Center,
		FontSize = 30,
		Text = "plantNamePlaceholder"
	};
	Label moistMeterLabel = new()
	{
		HorizontalOptions = LayoutOptions.Center,
		VerticalOptions = LayoutOptions.Center,
		FontSize = 25,
		Text = "MoistMeter:"
	};
	ProgressBar moistnessBar = new()
	{
		Progress = Networking.progress,
		ProgressColor = Color.FromRgb(255, 128, 0)
	};
	Thread networkLoopThread = new Thread(new ThreadStart(NetworkLoop));


	public PlantCard()
	{
		Content = mainFrame;
		mainFrame.Content = mainLayout;
		mainLayout.Children.Add(plantNameLabel);
		mainLayout.Children.Add(moistMeterLabel);
		mainLayout.Children.Add(moistnessBar);
		moistnessBar.Loaded += async delegate
		{
			while (true)
			{
				moistnessBar.Progress = Networking.progress;
				System.Diagnostics.Debug.WriteLine($"progress: {moistnessBar.Progress}");
				await Task.Delay(5000);
			}
		};
		networkLoopThread.Start();
	}

	public void testfunc(object sender, EventArgs args)
	{
		moistnessBar.ProgressTo(Networking.progress, 100, Easing.Linear);
		System.Diagnostics.Debug.WriteLine("srsef");
	}
}
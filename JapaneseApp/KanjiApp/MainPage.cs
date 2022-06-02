﻿namespace KanjiApp;

public class MainPage : ContentPage
{
	public MainPage()
	{
		Networking.OpenConnection(Networking.localHost);
		int count = 0;
		string[] currentSentence;
		bool isJLPTN5EXTD = true;
		//bool 

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
			Text = "この漢字の読み方を書いて:"
		};
		Label kanjiLabel = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center,
			FontSize = 35,
			Text = $"Current count: {count}"
		};
		Entry kanjiReadingEntry = new()
		{
			HorizontalOptions = LayoutOptions.Center,
			VerticalOptions = LayoutOptions.Center
		};
		Content = scrollView;
		scrollView.Content = verticalStackLayout;
		verticalStackLayout.Children.Add(header);
		verticalStackLayout.Children.Add(kanjiLabel);
		verticalStackLayout.Children.Add(kanjiReadingEntry);
		kanjiLabel.Loaded += (sender, args) =>
		{
			currentSentence = Networking.RetreiveData().Split(',');
			kanjiLabel.Text = currentSentence[0];
		};
		kanjiReadingEntry.Completed += (sender, args) =>
		{
			count++;
			Networking.SendData("2:" + kanjiReadingEntry.Text);
			kanjiReadingEntry.UpdateText("");
		};
	}
}
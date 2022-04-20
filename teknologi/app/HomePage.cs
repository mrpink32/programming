using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace app
{
	public class HomePage : ContentPage
	{
		private UInt32 x;
		private void IncreaseCounter()
		{
			x++;
		}



		public HomePage()
		{
			Label header = new()
			{
				HorizontalOptions = LayoutOptions.Center,
				FontAttributes = FontAttributes.Bold,
				Text = "Plant overview:",
				FontSize = 50,
			};

			ScrollView scrollView = new()
			{
				

			};


			Title = "Plant app demo";
			Content = new StackLayout
			{
				Spacing = 25,
				Padding = 30,
				Children = {
					header,
					scrollView
				}
			};
		}
	}
}
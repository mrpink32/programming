using FreesicXam.ViewModels;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Xamarin.Forms;
using Xamarin.Forms.Xaml;

namespace FreesicXam.Views
{
    [XamlCompilation(XamlCompilationOptions.Compile)]
    public partial class GamerPage : ContentPage
    {
        public GamerPage()
        {
            InitializeComponent();

            //BindingContext = new GamerViewModel();
        }

        void hmmm(object sender, SelectedItemChangedEventArgs e)
        {
            testBox = sender as ListView;
            if (testBox != null)
            {
                Title = $"{testBox.SelectedItem}";
            }
        }

        private void testButton_Clicked(object sender, EventArgs e)
        {

        }
    }
}
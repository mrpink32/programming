using System;
using System.IO;
using System.Net.Sockets;
using System.Text;
using Xamarin.Forms;

namespace FreesicXam
{
    public partial class App : Application
    {
        

        public App()
        {
            InitializeComponent();

            MainPage = new AppShell();
        }

        protected override void OnStart()
        {

        }

        protected override void OnSleep()
        {
        }

        protected override void OnResume()
        {
        }


        

    }
}

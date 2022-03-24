using FreesicXam.Models;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Text;
using Xamarin.Forms;

namespace FreesicXam.ViewModels
{
    class GamerViewModel : BindableObject
    {
        public static Collection<Song> Song { get; }
        /*public static GamerViewModel()
        {
            Song = new Collection<Song>();
        }*/

        public static void ReceiveSongNamesFromServer()
        {
            // Loop trough and receive stuffs
            // Receive string array
            /*for (int i = 0; i < length; i++)
            {

            }*/
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
            Song.Add(new Song { Name = "blah", ArtistName = "blah" });
        }
    }
}

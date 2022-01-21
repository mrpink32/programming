using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;
using CSCore;
using CSCore.Codecs.MP3;
using CSCore.SoundOut;

namespace FreesicClient
{
    public class MusicPlayer
    {
        private static IWaveSource source;
        private static readonly ISoundOut player = new DirectSoundOut();

        public static void UpdateSource(Stream stream)
        {
            source = source.AppendSource(source => new DmoMp3Decoder(stream));
            Console.WriteLine("Source updated!");
        }

        public static void PlayMusic(byte[] data)
        {
            if (player.PlaybackState.Equals(PlaybackState.Playing))
            {
                StopMusic();
            }
            MemoryStream stream = new(data);
            source = new DmoMp3Decoder(stream);
            player.Initialize(source);
            player.Play();
        }

        public static void PlayMusic(Stream stream)
        {
            if (player.PlaybackState.Equals(PlaybackState.Playing))
            {
                StopMusic();
            }
            source = new DmoMp3Decoder(stream);
            player.Initialize(source);
            player.Play();
        }

        public static void StopMusic()
        {
            player.Stop();
        }

        public static void ChangeVolume(double data)
        {
            player.Volume = (float)data;
        }

        //private static ISoundOut GetSoundOut()
        //{
        //    if (WasapiOut.IsSupportedOnCurrentPlatform)
        //        return new WasapiOut();
        //    else
        //        return new DirectSoundOut();
        //}

        //private static IWaveSource GetSoundSource(Stream stream)
        //{
        //    Instead of using the CodecFactory as helper, you specify the decoder directly:
        //    return new DmoMp3Decoder(stream);
        //}
    }
    public class Song
    {
        public string Name { get; set; }
        public string Author { get; set; }
        public byte[] Data { get; set; }

    }
    public class Playlist 
    {
        List<Song> Songs { get; set; }
    
    }
}

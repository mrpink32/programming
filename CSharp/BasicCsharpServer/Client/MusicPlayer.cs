using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;
using CSCore;
using CSCore.Codecs;
using CSCore.Codecs.MP3;
using CSCore.CoreAudioAPI;
using CSCore.SoundOut;
using CSCore.DirectSound;
using CSCore.Streams;


namespace Client
{
    public class MusicPlayer
    {
        private static IWaveSource source;
        public static ISoundOut player = new DirectSoundOut();

        public static WriteableBufferingSource src = new WriteableBufferingSource(new WaveFormat(48000, 32, 2)) { FillWithZeros = true };
        public static WriteableBufferingSource bufferingSource = new WriteableBufferingSource(new WaveFormat());
        public static Queue<byte[]> _q = new Queue<byte[]>();

        public static void UpdateSource(Stream stream)
        {
            source = source.AppendSource(source => new DmoMp3Decoder(stream));
            Console.WriteLine("Source updated!");
        }

        public static void PlayMusic(byte[] data)
        {
            MemoryStream stream = new MemoryStream(data);
            //bufferingSource = new WriteableBufferingSource(new WaveFormat());
            //bufferingSource.Write(data, 0, data.Length);
            source =  new DmoMp3Decoder(stream);
            player.Initialize(bufferingSource);
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

        public static void UpdateBufferedSource()
        {
            if (_q.Count > 0)
            {
                byte[] buffer = _q.Dequeue();

                bufferingSource.Write(buffer, 0, buffer.Length);
                Console.WriteLine($"Buffer written {buffer.Length} bytes");
            }
        }

        public static void PlayBufferedMusic()
        {
            UpdateBufferedSource();
            player.Initialize(bufferingSource);
            player.Play();
            while (player.PlaybackState.Equals(PlaybackState.Playing))
            {
                UpdateBufferedSource();
            }
        }

        public static void StopMusic()
        {
            player.Stop();
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

        public static void SetupBufferingSource()
        {
            //WriteableBufferingSource src = new WriteableBufferingSource(new WaveFormat(48000, 32, 2)) { FillWithZeros = true };
            player.Initialize(src);
            player.Play();
        }
    }
}

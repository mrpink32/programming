using System;
using System.Collections.Generic;
using System.Text;
using System.IO;

namespace Client
{
    public class CreateAudioFile
    {
        public static FileStream CreateMP3(byte[] data, string outputName)
        {
            using (FileStream bytesToAudio = File.Create($"D:/Github/BasicCsharpServer/Client/Temp/{outputName}.mp3"))
            {
                bytesToAudio.Write(data, 0, data.Length);
                Stream audioFile = bytesToAudio;
                bytesToAudio.Close();
                return (FileStream)audioFile;
            }
        }

        public static FileStream CreateMP3(MemoryStream stream, string outputName)
        {
            using (FileStream bytesToAudio = File.Create($"D:/Github/BasicCsharpServer/Client/Temp/{outputName}.mp3"))
            {
                byte[] data = stream.ToArray();
                bytesToAudio.Write(data, 0, data.Length);
                Stream audioFile = bytesToAudio;
                bytesToAudio.Close();
                return (FileStream)audioFile;
            }
        }
    }
}

using Microsoft.Maui.Controls;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.IO;


namespace FreesicClient
{
    public class MainPage : ContentPage
    {
        public MainPage(NetworkManager nw)
        {
            // Do networking stuff
            nw.outputText.Add(nw.ReceiveString());
            nw.SendString("connection check");
            nw.commands = nw.ReceiveStringArray();
            nw.songList = nw.ReceiveStringArray();
            nw.outputText.Add("type \"help\" to view a list of available commands");

            DateTime startTime;
            DateTime endTime;

            ColumnDefinition column1 = new();
            ColumnDefinition column2 = new();
            RowDefinition row1 = new();
            RowDefinition row2 = new();
            // Do visual stuff
            Grid mainView = new()
            {
                VerticalOptions = LayoutOptions.FillAndExpand,
                HorizontalOptions = LayoutOptions.FillAndExpand,
            };
            
            mainView.ColumnDefinitions.Add(column1);
            mainView.ColumnDefinitions.Add(column2);
            mainView.RowDefinitions.Add(row1);
            mainView.RowDefinitions.Add(row2);


            Entry inputBox = new()
            {
            };
            mainView.SetColumn(inputBox, 0);
            mainView.SetRow(inputBox, 0);
            inputBox.Completed += Entry_Completed;
            ListView terminal = new()
            {
                ItemsSource = nw.outputText,
            };
            mainView.SetColumn(terminal, 0);
            mainView.SetRow(terminal, 1);
            ListView songDisplay = new()
            {
                ItemsSource = nw.songList,
            };
            mainView.SetColumn(songDisplay, 1);
            mainView.SetRowSpan(songDisplay, 2);
            songDisplay.ItemSelected += ListView_ItemSelected;

            // add children to main view and set as content of page
            mainView.Children.Add(inputBox);
            mainView.Children.Add(terminal);
            mainView.Children.Add(songDisplay);
            Content = mainView;

            terminal.BeginRefresh();

            void Log(string message)
            {

            }

            void ListView_ItemSelected(object sender, SelectedItemChangedEventArgs e)
            {
                songDisplay = sender as ListView;
                if (songDisplay != null)
                {
                    Title = songDisplay.SelectedItem.ToString();
                    nw.SendString("music");
                    nw.SendString(songDisplay.SelectedItem.ToString());
                    nw.outputText.Add($"Download started at: {startTime = DateTime.Now:HH:mm:ss}");
                    MemoryStream audio = nw.ReceiveMusic();
                    nw.outputText.Add($"Download finished at: {endTime = DateTime.Now:HH:mm:ss}");
                    nw.outputText.Add($"Download took: \n{endTime - startTime:mm}: minutes and \n{endTime - startTime:ss}: seconds");
                    MusicPlayer.PlayMusic(audio);
                }
            }

            void Entry_Completed(object sender, EventArgs e)
            {
                string input = ((Entry)sender).Text;
                nw.outputText.Add(input);
                HandleCommands(input);
            }

            void HandleCommands(string command)
            {
                if (command.Equals(nw.commands[2])) { nw.SendString(command); nw.CloseConnection(); }
                else if (command.Equals(nw.commands[0]))
                {
                    //Console.WriteLine("commands:");
                    nw.outputText.Add("commands:");
                    for (int i = 0; i < nw.commands.Length; i++)
                    {
                        //Console.WriteLine($"{i + 1}: {commands[i]}");
                        nw.outputText.Add($"{i + 1}: {nw.commands[i]}");
                    }
                }
                //else if (command.Equals(nw.commands[1]))
                //{
                //    nw.SendString(command);
                //    foreach (string item in nw.songList)
                //    {
                //        nw.outputText.Add($"{item}");
                //    }
                //    nw.SendString($"{Console.ReadLine()}");
                //    nw.outputText.Add($"Download started at: {startTime = DateTime.Now:HH:mm:ss}");
                //    MemoryStream audio = nw.ReceiveMusic();
                //    nw.outputText.Add($"Download finished at: {endTime = DateTime.Now:HH:mm:ss}");
                //    nw.outputText.Add($"Download took: \n{endTime - startTime:mm}: minutes and \n{endTime - startTime:ss}: seconds");
                //    MusicPlayer.PlayMusic(audio);
                //}
                else if (command == "stop")
                {
                    MusicPlayer.StopMusic();
                }
                else { nw.SendString(command); }
            }
        }
    }
}
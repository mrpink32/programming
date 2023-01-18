#include "resource.h"
#include <windows.h>
#include <iostream>
#include <shobjidl.h>

#define MAIN_EDIT 101
#define MAIN_BUTTON 102

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

BOOL LoadTextFileToEdit(HWND hEdit, LPCTSTR pszFileName)
{
    HANDLE hFile;
    BOOL bSuccess = FALSE;

    hFile = CreateFile(pszFileName, GENERIC_READ, FILE_SHARE_READ, NULL,
                       OPEN_EXISTING, 0, NULL);
    if (hFile != INVALID_HANDLE_VALUE)
    {
        DWORD dwFileSize;

        dwFileSize = GetFileSize(hFile, NULL);
        if (dwFileSize != 0xFFFFFFFF)
        {
            LPWSTR pszFileText;

            pszFileText = (LPWSTR)GlobalAlloc(GPTR, dwFileSize + 1);
            if (pszFileText != NULL)
            {
                DWORD dwRead;

                if (ReadFile(hFile, pszFileText, dwFileSize, &dwRead, NULL))
                {
                    pszFileText[dwFileSize] = 0; // Add null terminator
                    if (SetWindowText(hEdit, pszFileText))
                        bSuccess = TRUE; // It worked!
                }
                GlobalFree(pszFileText);
            }
        }
        CloseHandle(hFile);
    }
    return bSuccess;
}

BOOL SaveTextFileFromEdit(HWND hEdit, LPCTSTR pszFileName)
{
    HANDLE hFile;
    BOOL bSuccess = FALSE;

    hFile = CreateFile(pszFileName, GENERIC_WRITE, 0, NULL,
                       CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile != INVALID_HANDLE_VALUE)
    {
        DWORD dwTextLength;

        dwTextLength = GetWindowTextLength(hEdit);
        // No need to bother if there's no text.
        if (dwTextLength > 0)
        {
            DWORD dwBufferSize = dwTextLength + 1;

            LPWSTR pszText;
            pszText = (LPWSTR)GlobalAlloc(GPTR, dwBufferSize);
            if (pszText != NULL)
            {
                if (GetWindowText(hEdit, pszText, dwBufferSize))
                {
                    DWORD dwWritten;

                    if (WriteFile(hFile, pszText, dwTextLength, &dwWritten, NULL))
                        bSuccess = TRUE;
                }
                GlobalFree(pszText);
            }
        }
        CloseHandle(hFile);
    }
    return bSuccess;
}

LRESULT WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
{
    switch (uMsg)
    {
    case WM_LBUTTONDOWN:
    {
        wchar_t szFileName[MAX_PATH];
        HINSTANCE hInstance = GetModuleHandleW(NULL);

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

        HWND hWndButton = CreateWindowW(
            L"BUTTON",                                                    // Predefined class; Unicode assumed
            L"OK",                                                        // Button text
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON | BS_FLAT, // Styles
            0,                                                            // x position
            0,                                                            // y position
            50,                                                           // Button width
            50,                                                           // Button height
            hwnd,                                                         // Parent window
            (HMENU)MAIN_BUTTON,                                           // No menu.
            GetModuleHandleW(NULL),
            NULL); // Pointer not needed.
        if (hWndButton == NULL)
        {
            MessageBoxW(hwnd, L"Could not create edit box.", L"Error", MB_OK | MB_ICONERROR);
            return 0;
        }

        WindowSize windowSize = GetWindowSize(hwnd);
        HWND hEdit = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", NULL,
                                     WS_CHILD | WS_VISIBLE | WS_VSCROLL | ES_MULTILINE | ES_AUTOVSCROLL, 50, 0, windowSize.windowWidth,
                                     windowSize.windowHeight, hwnd, (HMENU)MAIN_EDIT, GetModuleHandleW(NULL), NULL);
        if (hEdit == NULL)
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
        SetWindowPos(textArea, NULL, 50, 0, windowSize.windowWidth, windowSize.windowHeight, SWP_NOZORDER);

        HWND button = GetDlgItem(hwnd, MAIN_BUTTON);
        if (button == NULL)
        {
            std::cout << "Failed to get button handle" << std::endl;
        }
        SetWindowPos(button, NULL, 0, 0, 50, 50, SWP_NOZORDER);

        break;
    }
    // case WM_PAINT:
    //{
    //     PAINTSTRUCT ps;
    //     HDC hdc = BeginPaint(hwnd, &ps);
    //     HBRUSH hBrush = CreateSolidBrush(RGB(0, 0, 0));
    //     FillRect(hdc, &ps.rcPaint, hBrush);
    //     //for (size_t i = 0; i < 1080; i++)
    //     //{
    //     //    for (size_t j = 0; j < 1920; j++)
    //     //    {
    //     //        // Rectangle(hdc, i, j, i + 1, j + 1);
    //     //        SetPixel(hdc, i, j, RGB(255, 0, 255));
    //     //    }
    //     //}
    //     DeleteObject(hBrush);
    //     EndPaint(hwnd, &ps);
    //     return 0;
    // }
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
            OPENFILENAME ofn;
            wchar_t szFileName[MAX_PATH] = L"";

            ZeroMemory(&ofn, sizeof(ofn));

            ofn.lStructSize = sizeof(OPENFILENAME); // SEE NOTE BELOW
            ofn.hwndOwner = hwnd;
            ofn.lpstrFilter = L"Text Files (*.txt)\0*.txt\0All Files (*.*)\0*.*\0";
            ofn.lpstrFile = szFileName;
            ofn.nMaxFile = MAX_PATH;
            ofn.Flags = OFN_FILEMUSTEXIST | OFN_HIDEREADONLY;
            ofn.lpstrDefExt = L"";

            if (GetOpenFileNameW(&ofn))
            {
                // Do something usefull with the filename stored in szFileName
                HWND textArea = GetDlgItem(hwnd, MAIN_EDIT);
                LoadTextFileToEdit(textArea, szFileName);
            }
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
    windowClass.hIcon = LoadIconW(hInstance, MAKEINTRESOURCEW(IDI_MYICON));
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

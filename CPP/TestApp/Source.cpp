#include <windows.h>
#include <iostream>

// #define 

const wchar_t CLASS_NAME[] = L"ServerManagerWindowClass";




struct StateInfo {
    // ... (struct members not shown)
};

inline StateInfo* GetAppState(HWND hwnd)
{
    LONG_PTR ptr = GetWindowLongPtr(hwnd, GWLP_USERDATA);
    StateInfo* pState = reinterpret_cast<StateInfo*>(ptr);
    return pState;
}

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
{
    StateInfo* pState;
    if (uMsg == WM_CREATE)
    {
        CREATESTRUCT* pCreate = reinterpret_cast<CREATESTRUCT*>(lParam);
        pState = reinterpret_cast<StateInfo*>(pCreate->lpCreateParams);
        SetWindowLongPtr(hwnd, GWLP_USERDATA, (LONG_PTR)pState);

        // Creates menu and sub-menu
        HMENU hMenu = CreateMenu();
        HMENU hFileMenu = CreateMenu();

        // Appends to initial menu line
        AppendMenuW(hMenu, MF_STRING, 1, L"Files");
        AppendMenuW(hMenu, MF_POPUP, (UINT_PTR)hFileMenu, L"Options");
        
        // Appends to sub-menu
        AppendMenuW(hFileMenu, MF_STRING, 2, L"Lol get trolled");
        AppendMenuW(hFileMenu, MF_SEPARATOR, NULL, NULL);
        AppendMenuW(hFileMenu, MF_STRING, 3, L"Grim");

        // Sets the menu to the window
        SetMenu(hwnd, hMenu);
    }
    else
    {
        pState = GetAppState(hwnd);
    }
    switch (uMsg)
    {
    case WM_CLOSE:
        if (MessageBoxW(hwnd, L"Are you sure you want to exit?", L"Exit prompt", MB_YESNO) == IDYES)
        {
            DestroyWindow(hwnd);
        }
        return 0;
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    case WM_SIZE:
    {
        RECT rect;
        GetClientRect(hwnd, &rect);
        int windowWidth = rect.right - rect.left;
        int windowHeight = rect.bottom - rect.top;
        int bufferSize = windowWidth * windowHeight * sizeof(unsigned int);
    }
    return 0;
    case WM_PAINT:
    {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hwnd, &ps);
        FillRect(hdc, &ps.rcPaint, (HBRUSH)(COLOR_DESKTOP));
        EndPaint(hwnd, &ps);
    }
    return 0;
    case WM_COMMAND:
        switch (wParam)
        {
        case 1:
            MessageBeep(MB_OK);
            break;
        case 2:
            MessageBoxW(hwnd, L"Du er grim", L"Grim", MB_OK);
            break;
        case 3:
            MessageBoxW(hwnd, L"You have been gnomed", L"Truth", MB_OK);
            break;
        default:
            break;
        }
        return 0;
    default:
        return DefWindowProcW(hwnd, uMsg, wParam, lParam);
    }
    return TRUE;
}

int wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR args, int nCmdShow)
{
    // Create Window Class
    WNDCLASSW windowClass = {};
    windowClass.lpfnWndProc = WindowProc;
    windowClass.hInstance = hInstance;
    windowClass.lpszClassName = CLASS_NAME;
    // Register Window Class
    ATOM atom = RegisterClassW(&windowClass);
    if (atom == 0)
    {
        std::cout << "Failed to register WindowClass!" << std::endl;
        return 0;
    }

    StateInfo* pState = new (std::nothrow) StateInfo;
    if (pState == NULL)
    {
        return 0;
    }

    HWND hwnd = CreateWindowExW(
        0,
        CLASS_NAME,
        L"ServerManagerClient",
        WS_OVERLAPPEDWINDOW,

        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,

        NULL,
        NULL,
        hInstance,
        pState
    );
    if (hwnd == NULL)
    {
        std::cout << "Failed to create window!" << std::endl;
        return 0;
    }
    ShowWindow(hwnd, nCmdShow);
    MSG msg = {};
    while (GetMessageW(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    // MessageBoxW(NULL, L"This is a window", L"Window", MB_OK);
    return 0;
}


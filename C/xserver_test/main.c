#include <X11/Xlib.h>

int main()
{
    Display *testDisplay = XOpenDisplay(NULL);
    XWarpPointer(testDisplay, None, None, None, None, None, None, 500, 500);
}

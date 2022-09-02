#include <X11/Xlib.h>
#include <X11/Xresource.h>
#include <stdio.h>

int main()
{
    // Display *testDisplay = XOpenDisplay(NULL);
    // XWarpPointer(testDisplay, None, None, None, None, None, None, 500, 500);
    XColor *xcolors;
    Colormap colors;
    int32_t val = XQueryColors(NULL, colors, xcolors, 5);
    printf("%d", val);
}

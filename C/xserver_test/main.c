#include </usr/include/X11/Xlib.h>
#include </usr/include/X11/Xutil.h>
#include <stdio.h>

// REMEMBER TO COMPILE WITH "-lX11"!!!!

int main()
{
    XColor c;
    Display *d = XOpenDisplay((char *)NULL);

    int x = 0; // Pixel x
    int y = 0; // Pixel y

    XImage *image;
    image = XGetImage(d, XRootWindow(d, XDefaultScreen(d)), x, y, 1, 1, AllPlanes, XYPixmap);
    c.pixel = XGetPixel(image, 0, 0);
    XFree(image);
    XQueryColor(d, XDefaultColormap(d, XDefaultScreen(d)), &c);
    printf("%d, %d, %d\n", c.red / 256, c.green / 256, c.blue / 256);

    return 0;
}

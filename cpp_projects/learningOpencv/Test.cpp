#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/objdetect.hpp>
#include <iostream>

using namespace cv;

Mat img/* = imread("D:/GitHub/Cascade_Training/Tests/Uno (2).png")*/;
int scaleFactorBar = 1;
int neighbors = 10;
std::string scaleFactorText = "Scale factor: ";

int main()
{
	std::string cascadePath = "D:/GitHub/Cascade_Training/Cascades/CustomMade/cascade_Uno_green_1.xml";
	VideoCapture capture(0);

	CascadeClassifier unoCascade;
	unoCascade.load(cascadePath);

	std::vector<Rect> cards;

	namedWindow("Trackbars", (1600, 1000));
	createTrackbar("Scale Factor", "Trackbars", &scaleFactorBar, 10);
	createTrackbar("Neighbors", "Trackbars", &neighbors, 50);
	

	while (true)
	{

		capture.read(img);
		double scaleFactor = 1 + ((double)scaleFactorBar / 10);
		unoCascade.detectMultiScale(img, cards, scaleFactor, neighbors);
		std::cout << scaleFactor << std::endl;

		for (int i = 0; i < cards.size(); i++)
		{
			rectangle(img, cards[i].tl(), cards[i].br(), Scalar(255, 0, 255), 3);
			putText(img, "Uno card", Point(cards[i].tl().x, (cards[i].tl().y - 10)), FONT_HERSHEY_COMPLEX_SMALL, 0.8, Scalar(255, 0, 255), 2);
		}


		imshow("image", img);
		waitKey(1);
	}




	/*unoCascade.detectMultiScale(img, cards, 1.1, 10);

	for (int i = 0; i < cards.size(); i++)
	{
		rectangle(img, cards[i].tl(), cards[i].br(), Scalar(255, 0, 255), 3);
	}

	imshow("image", img);
	waitKey(0);*/

}
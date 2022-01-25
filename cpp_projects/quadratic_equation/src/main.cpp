#include <iostream>
#include <math.h>

void get_numeric_input(double &x, const char *text) {
	//https://stackoverflow.com/questions/10828937/how-to-make-cin-take-only-numbers
	std::cout << text;
	while (!(std::cin >> x)) {
		std::cin.clear();
		std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
		std::cout << text;
	}
}

void quadratic_equation1(double *a_p, double *b_p, double *c_p) {
	double d = (pow((*b_p), 2) - 4 * (*a_p) * (*c_p));
	double *d_p = &d;
	if (*d_p > 0) {
		double ans1 = (-(*b_p) + sqrt(*d_p)) / 2 * (*a_p);
		double ans2 = (-(*b_p) - sqrt(*d_p)) / 2 * (*a_p);
		std::cout << "This quadratic formula has 2 answers:" << std::endl;
		printf("1: %lf\n", ans1);
		printf("2: %lf\n", ans2);
	}
	else if (*d_p == 0) {
		double ans = (-(*b_p) + sqrt(*d_p)) / 2 * (*a_p);
		std::cout << "This quadratic formula has 1 answers:" << std::endl;
		printf("1: %lf\n", ans);
	}
	else {
		std::cout << "This quadratic formula has 0 answers!" << std::endl;
	}
}

void quadratic_equation2(double &a_ref, double &b_ref, double &c_ref) {
	double d = (pow((b_ref), 2) - 4 * (a_ref) * (c_ref));
	double *d_p = &d;
	if (*d_p > 0) {
		double ans1 = (-(b_ref) + sqrt(*d_p)) / 2 * (a_ref);
		double ans2 = (-(b_ref)-sqrt(*d_p)) / 2 * (a_ref);
		std::cout << "This quadratic formula has 2 answers:" << std::endl;
		printf("1: %lf\n", ans1);
		printf("2: %lf\n", ans2);
	}
	else if (*d_p == 0) {
		double ans = (-(b_ref) + sqrt(*d_p)) / 2 * (a_ref);
		std::cout << "This quadratic formula has 1 answers:" << std::endl;
		printf("1: %lf\n", ans);
	}
	else {
		std::cout << "This quadratic formula has 0 answers!" << std::endl;
	}
}

int main() {
	double a, b, c;
	while (true) {
		get_numeric_input(a, "Input a: ");
		get_numeric_input(b, "Input b: ");
		get_numeric_input(c, "Input c: ");
		quadratic_equation1(&a, &b, &c);
		quadratic_equation2(a, b, c);
		while (true) {
			std::cout << "Try again? [y/n]: " << std::endl;
			char input;
			std::cin >> input;
			/*switch (input) {
			case 'y':
				std::cout << "y er sej" << std::endl;
				break;
			case 'n':
				std::cout << "n er sej" << std::endl;
				return 0;
			default:
				std::cout << "Illegal action, input has to be a valid key..." << std::endl;
				continue;
			}*/
			if (input == 'y') { break; }
			else if (input == 'n') { return 0; }
			std::cout << "Illegal action, input has to be a valid key..." << std::endl;
		}
	}
}
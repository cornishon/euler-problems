// sum square difference
#include <iostream>
using namespace std;

int sum_squares(int n);
int square_sum(int n);

int main(int argc, char *argv[])
{
	cout << square_sum(100) - sum_squares(100) << endl;
	return 0;
}

int sum_squares(int n)
{
	int acc = 0;
	for (int i = 1; i <= n; i++)
		acc += i * i;
	return acc;
}

int square_sum(int n)
{
	int acc = 0;
	for (int i = 1; i <= n; i++)
		acc += i;
	return acc * acc;
}

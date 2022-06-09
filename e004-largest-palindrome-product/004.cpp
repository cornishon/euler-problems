// largest palindrome product
#include <iostream>
using namespace std;

bool is_pal(int n);

int main(int argc, char *argv[])
{
	int i = 99;
	int j = 99;
	int prod;

	// make sure that j is not too small
	// so we can break on 1st match
	// maybe think of a more intelligent condition?
	for (int i = 999; i > 99; i--)
		for (int j = 999; j > 0.9 * i; j--)
		{
			prod = i * j;
			if (is_pal(prod))
			{
				cout << i << " " << j << " " << prod;
				cout << endl;
				i = 1;
				j = 1;			// break 
			}
		}
	return 0;
}

bool is_pal(int n)
{
	int m = 0;
	for (int k = n; k != 0; k /= 10)
	{
		m *= 10;
		m += (k % 10);
	}
	return n == m;
};

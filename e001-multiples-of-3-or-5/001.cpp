// multiples of 3 or 5
#include <iostream>

int main(int argc, char *argv[])
{
	int x = 1;
	int sum = 0;
	
	while (x < 1000)
	{
		if (x % 3 == 0 || x % 5 == 0)
			sum += x;
		x++;
	}
	
	std::cout << sum << std::endl;
	
	return 0;
}
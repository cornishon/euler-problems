// even Fibonacci numbers
#include <iostream>

int main(int argc, char *argv[])
{
	int f1 = 1;
	int f2 = 2;
	int sum = 0;
	int tmp;
	
	while (f2 < 4000000)
	{
		if (f2 % 2 == 0)
			sum += f2;
		
		tmp = f1 + f2;
		f1 = f2;
		f2 = tmp;
	}
	
	std::cout << sum << std::endl;
}
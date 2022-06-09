//  largest prime factor
#include <iostream>

int main(int argc, char *argv[])
{
	long n =  600851475143;
	long ans;
	
	for (long i = 2; i <= n; i++)
	{
		if (n % i == 0)
		{
			ans = i;
			n /= i;
			// get rid of powers of i
			while (true)
			{
				if (n % i == 0) { n /= i; } else { break; }
			}
		}
	}
	std::cout << ans;
	return 0;
}
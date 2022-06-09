// Longest Collatz Sequence
#include <iostream>
using namespace std;

size_t collatz_len(long long n) {
	size_t len = 1;
	for (; n != 1; len++) {
		n = (n % 2) ? 3*n+1 : n/2;
	}
	return len;
}

int main() {
	size_t longest = 0;
	size_t len;
	for (long long n = 1; n < 1e6; ++n) {
		len = collatz_len(n);
		if (longest < len) {
			longest = len;
			cout <<"n = " << n << ": length = " << len << endl;
		}
	}
	return 0;
}
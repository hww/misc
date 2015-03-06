#include <stdint.h>
#include <stdlib.h>

size_t ConsistentHash(uint64_t key, size_t n) {
	uint64_t b = 1, j = 0;
	while (j < n) {
		b = j;
		key = key * 2862933555777941757ull + 1;
		j = uint64_t((b + 1) * (double(1u << 31) / double((key >> 33) + 1)));
	}
	return (size_t)b;
}


#include <vector>
#include <map>
#include <set>
typedef std::set<size_t> IntSet;
typedef std::map<size_t, IntSet> Map;


void put(const Map& m)
{
	for (const auto& p : m) {
		printf("hash[%d] ", (int)p.first);
		for (size_t x : p.second) {
			printf("%d ", (int)x);
		}
		printf("\n");
	}
}

Map make(size_t n, size_t N)
{
	Map m;
	for (size_t key = 0; key < N; key++) {
		m[ConsistentHash(key, n)].insert(key);
	}
//	put(m);
	return m;
}

int diff(const Map& m1, const Map& m2)
{
	int moved = 0;
	for (const auto& p : m1) {
//		printf("hash[%d] ", p.first);
		auto i2 = m2.find(p.first)->second;
		int n = 0;
		for (size_t x : p.second) {
			if (i2.find(x) == i2.end()) n++;
		}
//		printf("%d\n", n);
		moved += n;
	}
	printf("%d\n", moved);
	return moved;
}

int main(int argc, char *argv[])
{
	const int N = argc >= 2 ? atoi(argv[1]) : 10000;
	const int n1 = argc >= 3 ? atoi(argv[2]) : 100;
	const int n2 = argc >= 4 ? atoi(argv[3]) : 120;
	auto m1 = make(n1, N);
	puts("--");
	auto m2 = make(n2, N);
	int moved = diff(m1, m2);
	printf("expect %d %d\n", int((1 - n1 / double(n2)) * N), moved);
}

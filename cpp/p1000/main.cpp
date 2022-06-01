#include <iostream>
#define FAST std::ios_base::sync_with_stdio(false), std::cin.tie(nullptr), std::cout.tie(nullptr)

int main() {
    FAST;
    int a, b;
    std::cin >> a >> b;

    std::cout << a + b;

    return 0;
}
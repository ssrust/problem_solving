#include <cstdint>
#include <cstdio>
#include <utility>
#include <array>
#include <algorithm>
#define FAST std::ios_base::sync_with_stdio(false), std::cin.tie(nullptr), std::cout.tie(nullptr)

using digit_t = uint8_t;

// Double Digits
using ddigits_t = std::pair<digit_t, digit_t>;

static constexpr ddigits_t get_next_val(ddigits_t dd) {
    return {dd.second, (dd.first + dd.second) % 10};
}

static constexpr int ddtoi(ddigits_t dd) {
    return dd.first * 10 + dd.second;
}

static constexpr ddigits_t itodd(uint8_t x) {
    return {x/10, x%10};
}

static constexpr auto get_seq_map() {
    std::array<uint8_t, 100> ret{};

    for (uint8_t i = 0; i < ret.size(); i++) {
        ret[i] = ddtoi(get_next_val(itodd(i)));
    }

    return ret;
}

static constexpr auto get_result_map() {
    std::array<size_t, 100> ret{};
    std::ranges::fill(ret, -1);

    auto seq_map = get_seq_map();

    for (size_t i = 0; i < ret.size(); i++) {
        size_t cnt = 1;
        for (auto now = seq_map[i]; now != i; now = seq_map[now], cnt++);
        ret[i] = cnt;
    }

    return ret;
}

int main() {
    static constexpr auto result_map = get_result_map();

    size_t x;
    std::scanf("%d", &x);

    std::printf("%d", result_map[x]);

    return 0;
}
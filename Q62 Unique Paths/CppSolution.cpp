#include <algorithm> // std::min
#include <numeric>   // std::gcd
using namespace std;

class Solution {
public:
    int uniquePaths(int m, int n) {
        long long total = m + n - 2;                  // total moves
        long long k = min<long long>(m - 1, n - 1);   // choose the smaller side
        long long res = 1;

        for (long long i = 1; i <= k; ++i) {
            long long num = total - (k - i); // current numerator term
            long long den = i;               // current denominator term

            // Reduce numerator and denominator first
            long long g1 = std::gcd(num, den);
            num /= g1; den /= g1;

            // Also reduce with current result to avoid overflow
            long long g2 = std::gcd(res, den);
            res /= g2; den /= g2;

            // Now safe to multiply then divide in 64-bit
            res *= num;        // fits since num <= 198 and final result <= 2e9
            if (den != 1) res /= den; // exact division
        }
        return static_cast<int>(res); // guaranteed <= 2e9
    }
};

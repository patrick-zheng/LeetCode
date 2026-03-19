#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int nextNum(int n) {
        int total = 0;
        while (n > 0) {
            int digit = n % 10;
            total += digit * digit;
            n /= 10;
        }
        return total;
    }

    bool isHappy(int n) {
        int slow = n;
        int fast = nextNum(n);

        while (fast != 1 && slow != fast) {
            slow = nextNum(slow);
            fast = nextNum(nextNum(fast));
        }

        return fast == 1;
    }
};

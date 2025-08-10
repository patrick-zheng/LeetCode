class Solution {
public:
    double myPow(double x, int n) {
        long long power = n; // Avoid overflow for INT_MIN
        if (power < 0) {
            x = 1 / x;
            power = -power;
        }
        
        double result = 1.0;
        while (power > 0) {
            if (power & 1) {
                result *= x;
            }
            x *= x;
            power >>= 1;
        }
        return result;
    }
};

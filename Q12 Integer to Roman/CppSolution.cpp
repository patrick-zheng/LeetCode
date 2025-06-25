#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution {
public:
    string intToRoman(int num) {
        vector<pair<int, string>> romanNumerals = {
            {1000, "M"}, {900, "CM"}, {500, "D"}, {400, "CD"},
            {100, "C"}, {90, "XC"}, {50, "L"}, {40, "XL"},
            {10, "X"}, {9, "IX"}, {5, "V"}, {4, "IV"}, {1, "I"}
        };

        string result = "";
        for (auto& [value, symbol] : romanNumerals) {
            int count = num / value;  // How many times we can add this symbol
            for (int i = 0; i < count; ++i) {
                result += symbol;
            }  // Append the full symbol, 'count' times
            num -= value * count;  // Decrease the number by the total value we've processed
        }
        return result;
    }
};

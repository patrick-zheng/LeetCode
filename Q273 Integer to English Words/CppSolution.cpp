#include <string>
#include <vector>

using namespace std;

class Solution {
    static const vector<string>& below20() {
        static const vector<string> w = {
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
            "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
            "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        };
        return w;
    }

    static const vector<string>& tensWords() {
        static const vector<string> w = {
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
            "Eighty", "Ninety",
        };
        return w;
    }

    string chunkToWords(int n) const {
        if (n == 0) return "";
        if (n < 20) return below20()[n];
        if (n < 100) {
            string s = tensWords()[n / 10];
            if (n % 10) s += " " + below20()[n % 10];
            return s;
        }
        string s = below20()[n / 100] + " Hundred";
        if (n % 100) s += " " + chunkToWords(n % 100);
        return s;
    }

public:
    string numberToWords(int num) {
        if (num == 0) return "Zero";

        const vector<string> units = {"", "Thousand", "Million", "Billion"};
        vector<string> parts;
        int value = num;
        for (int i = 0; i < 4; ++i) {
            int chunk = value % 1000;
            if (chunk) {
                string part = chunkToWords(chunk);
                if (!units[i].empty()) part += " " + units[i];
                parts.push_back(part);
            }
            value /= 1000;
        }

        string result;
        for (int i = static_cast<int>(parts.size()) - 1; i >= 0; --i) {
            if (!result.empty()) result += " ";
            result += parts[i];
        }
        return result;
    }
};

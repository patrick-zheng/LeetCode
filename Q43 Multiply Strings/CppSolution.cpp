#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    string multiply(string num1, string num2) {
        if (num1 == "0" || num2 == "0") return "0";

        int len1 = num1.size(), len2 = num2.size();
        vector<int> result(len1 + len2, 0);

        for (int i = len1 - 1; i >= 0; i--) {
            for (int j = len2 - 1; j >= 0; j--) {
                int mul = (num1[i] - '0') * (num2[j] - '0');
                int p1 = i + j, p2 = i + j + 1;
                int total = mul + result[p2];

                result[p2] = total % 10;
                result[p1] += total / 10;
            }
        }

        string resultStr;
        for (int num : result) {
            if (!(resultStr.empty() && num == 0)) {
                resultStr.push_back(num + '0');
            }
        }

        return resultStr.empty() ? "0" : resultStr;
    }
};

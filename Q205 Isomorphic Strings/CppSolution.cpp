#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool isIsomorphic(string s, string t) {
        vector<int> sToT(256, -1), tToS(256, -1);

        for (int i = 0; i < s.size(); ++i) {
            unsigned char cs = s[i], ct = t[i];
            if (sToT[cs] == -1 && tToS[ct] == -1) {
                sToT[cs] = ct;
                tToS[ct] = cs;
            } else if (sToT[cs] != ct || tToS[ct] != cs) {
                return false;
            }
        }

        return true;
    }
};

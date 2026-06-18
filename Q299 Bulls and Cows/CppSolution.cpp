#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string getHint(string secret, string guess) {
        int bulls = 0;
        vector<int> secret_freq(10, 0);
        vector<int> guess_freq(10, 0);

        for (int i = 0; i < static_cast<int>(secret.size()); ++i) {
            if (secret[i] == guess[i]) {
                ++bulls;
            } else {
                ++secret_freq[secret[i] - '0'];
                ++guess_freq[guess[i] - '0'];
            }
        }

        int cows = 0;
        for (int d = 0; d < 10; ++d) {
            cows += min(secret_freq[d], guess_freq[d]);
        }
        return to_string(bulls) + "A" + to_string(cows) + "B";
    }
};

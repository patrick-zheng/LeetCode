#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        int n = gas.size();
        int total = 0;   // net gas
        int tank = 0;    // current gas in tank
        int start = 0;   // candidate starting index
        
        for (int i = 0; i < n; ++i) {
            int diff = gas[i] - cost[i];
            total += diff;
            tank  += diff;
            
            if (tank < 0) {
                // cannot start anywhere from previous start to i
                start = i + 1;
                tank = 0;
            }
        }
        
        return total >= 0 ? start : -1;
    }
};

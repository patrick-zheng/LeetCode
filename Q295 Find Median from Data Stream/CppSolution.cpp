#include <queue>
#include <vector>

using namespace std;

class MedianFinder {
    priority_queue<int> low;
    priority_queue<int, vector<int>, greater<int>> high;

public:
    MedianFinder() = default;

    void addNum(int num) {
        low.push(num);
        high.push(low.top());
        low.pop();
        if (low.size() < high.size()) {
            low.push(high.top());
            high.pop();
        }
    }

    double findMedian() {
        if (low.size() > high.size()) {
            return static_cast<double>(low.top());
        }
        return (static_cast<double>(low.top()) + high.top()) / 2.0;
    }
};

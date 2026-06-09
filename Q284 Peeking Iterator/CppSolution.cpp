/*
 * Below is the interface for Iterator, you should write your solution to use it.
 *
 * class Iterator {
 *     struct Data;
 *     Data* data;
 * public:
 *     Iterator(const vector<int>& nums);
 *     int next();
 *     bool hasNext() const;
 * };
 */

class PeekingIterator : public Iterator {
    bool has_cached;
    int cached;

public:
    PeekingIterator(const vector<int>& nums) : Iterator(nums) {
        has_cached = Iterator::hasNext();
        if (has_cached) {
            cached = Iterator::next();
        }
    }

    int peek() {
        return cached;
    }

    int next() {
        int result = cached;
        has_cached = Iterator::hasNext();
        if (has_cached) {
            cached = Iterator::next();
        }
        return result;
    }

    bool hasNext() const {
        return has_cached;
    }
};

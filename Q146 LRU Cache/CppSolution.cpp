#include <unordered_map>
#include <list>

using namespace std;

class Solution {
    class LRUCache {
        int capacity;
        list<pair<int,int>> cache;
        unordered_map<int, list<pair<int,int>>::iterator> mp;
    
    public:
        LRUCache(int capacity) {
            this->capacity = capacity;
        }
        
        int get(int key) {
            if (mp.find(key) == mp.end())
                return -1;

            cache.splice(cache.begin(), cache, mp[key]);
            return mp[key]->second;
        }

        void put(int key, int value) {
            if (mp.find(key) != mp.end()) {
                cache.splice(cache.begin(), cache, mp[key]);
                mp[key]->second = value;
                return;
            }

            cache.emplace_front(key, value);
            mp[key] = cache.begin();

            if (cache.size() > capacity) {
                auto lru = cache.back();
                mp.erase(lru.first);
                cache.pop_back();
            }
        }
    };
};

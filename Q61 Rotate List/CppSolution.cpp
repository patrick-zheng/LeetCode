#include <vector>
#include <string>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if (!head || !head->next || k == 0) return head;

        // 1) Length
        int n = 1;
        ListNode* tail = head;
        while (tail->next) {
            tail = tail->next;
            ++n;
        }

        k %= n;
        if (k == 0) return head;

        // 2) Make circular, then cut
        tail->next = head;
        int steps = n - k - 1;           // new tail index from old head (0-based)
        ListNode* newTail = head;
        while (steps--) newTail = newTail->next;
        ListNode* newHead = newTail->next;
        newTail->next = nullptr;
        return newHead;
    }
};

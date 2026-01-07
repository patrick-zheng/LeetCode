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
    void reorderList(ListNode* head) {
        if (!head || !head->next || !head->next->next) return;

        // 1) Find middle
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
        }

        // 2) Reverse second half
        ListNode* second = slow->next;
        slow->next = nullptr; // split

        ListNode* prev = nullptr;
        while (second) {
            ListNode* nxt = second->next;
            second->next = prev;
            prev = second;
            second = nxt;
        }
        second = prev;

        // 3) Merge alternating
        ListNode* first = head;
        while (second) {
            ListNode* fNext = first->next;
            ListNode* sNext = second->next;

            first->next = second;
            second->next = fNext;

            first = fNext;
            second = sNext;
        }
    }
};

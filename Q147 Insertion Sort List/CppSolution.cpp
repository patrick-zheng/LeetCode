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
    ListNode* insertionSortList(ListNode* head) {
        if (!head) return head;

        ListNode dummy(0);
        dummy.next = head;

        ListNode* last_sorted = head;
        ListNode* curr = head->next;

        while (curr) {
            if (last_sorted->val <= curr->val) {
                last_sorted = last_sorted->next;
            } else {
                ListNode* prev = &dummy;
                while (prev->next && prev->next->val <= curr->val) {
                    prev = prev->next;
                }

                last_sorted->next = curr->next;
                curr->next = prev->next;
                prev->next = curr;
            }
            curr = last_sorted->next;
        }

        return dummy.next;
    }
};
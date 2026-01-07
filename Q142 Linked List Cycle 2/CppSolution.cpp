#include <vector>
#include <string>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        if (!head) return nullptr;

        ListNode* slow = head;
        ListNode* fast = head;

        // Step 1: Detect if a cycle exists
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;

            if (slow == fast) {
                break;
            }
        }

        // No cycle found
        if (!fast || !fast->next) {
            return nullptr;
        }

        // Step 2: Find the entry point of the cycle
        slow = head;
        while (slow != fast) {
            slow = slow->next;
            fast = fast->next;
        }

        return slow; // Start of the cycle
    }
};

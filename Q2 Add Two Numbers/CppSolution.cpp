#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
#include <stdexcept>

/*
 * Problem: https://leetcode.com/problems/add-two-numbers/
 * Solution: https://leetcode.com/problems/add-two-numbers/solutions/2958/two-sum/
 * Time Complexity: O(max(n, m))
 * Space Complexity: O(max(n, m))
 */

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};


using namespace std;

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode* answer = new ListNode();
        ListNode* current = answer;
        int carry = 0;

        while (l1 || l2 || carry) {
            int total = carry;
            if (l1) {
                total += l1->val;
                l1 = l1->next;
            }
            if (l2) {
                total += l2->val;
                l2 = l2->next;
            }
            carry = total / 10;
            current->next = new ListNode(total % 10);
            current = current->next;
        }
        return answer->next;
    }
};

/*
 * Problem: https://leetcode.com/problems/add-two-numbers/
 * Solution: https://leetcode.com/problems/add-two-numbers/solutions/
 * Time Complexity: O(max(n, m))
 * Space Complexity: O(max(n, m))
 */


// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}


export class Solution {
    addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
        const answer: ListNode = new ListNode(0);
        let current = answer;
        let carry = 0;

        while (l1 || l2 || carry) {
            const l1Value = l1 ? l1.val : 0;
            const l2Value = l2 ? l2.val : 0;

            const total = l1Value + l2Value + carry;
            carry = Math.floor(total / 10);
            current.next = new ListNode(total % 10);
            current = current.next;

            l1 = l1 ? l1.next : null;
            l2 = l2 ? l2.next : null;
        }

        return answer.next;
    }
}
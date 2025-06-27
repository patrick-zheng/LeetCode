// Definition for singly-linked list

class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
    const dummy = new ListNode(0);
    dummy.next = head;
    let fast: ListNode | null = dummy;
    let slow: ListNode | null = dummy;

    // Move fast pointer n+1 steps ahead
    for (let i = 0; i <= n; i++) {
        fast = fast!.next;
    }

    // Move both fast and slow pointers until fast reaches the end
    while (fast) {
        fast = fast.next;
        slow = slow!.next;
    }

    // Remove nth node from the end
    slow!.next = slow!.next!.next;

    return dummy.next;
};

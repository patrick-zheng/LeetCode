// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function swapPairs(head: ListNode | null): ListNode | null {
    if (head == null) { return head; }

    const dummy: ListNode = new ListNode();
    dummy.next = head;
    let curr: ListNode = dummy;

    while (curr.next && curr.next.next) {
        const first: ListNode = curr.next;
        const second: ListNode = first.next as ListNode;

        first.next = second.next;
        second.next = first;
        curr.next = second

        curr = first
    }
    return dummy.next;
};

// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function reverseKGroup(head: ListNode | null, k: number): ListNode | null {
    const dummy = new ListNode(0, head);
    let groupPrev = dummy;

    while (true) {
        let kth: ListNode | null = groupPrev;
        for (let i = 0; i < k && kth !== null; i++) {
            kth = kth.next;
        }
        if (!kth) break;

        const groupNext = kth.next;
        // reverse k nodes
        let prev: ListNode | null = groupNext;
        let curr: ListNode | null = groupPrev.next;

        for (let i = 0; i < k; i++) {
            const tmp = curr!.next;
            curr!.next = prev;
            prev = curr;
            curr = tmp;
        }

        const tmp = groupPrev.next;
        groupPrev.next = kth;
        groupPrev = tmp!;
    }

    return dummy.next;
};

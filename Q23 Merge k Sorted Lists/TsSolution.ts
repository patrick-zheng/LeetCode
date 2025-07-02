// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}


function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
    const heap: [number, number, ListNode][] = [];

    for (let i = 0; i < lists.length; i++) {
        if (lists[i]) {
            heap.push([lists[i]!.val, i, lists[i]!]);
        }
    }

    heap.sort((a, b) => a[0] - b[0]); // Initial heapify (min-heap)

    const dummy = new ListNode(0);
    let current = dummy;

    while (heap.length > 0) {
        const [_, i, node] = heap.shift()!;
        current.next = node;
        current = current.next;
        if (node.next) {
            // Re-insert next node into heap and keep sorted
            const newItem: [number, number, ListNode] = [node.next.val, i, node.next];
            const index = heap.findIndex(([v]) => v > node.next!.val);
            if (index === -1) heap.push(newItem);
            else heap.splice(index, 0, newItem);
        }
    }

    return dummy.next;
};

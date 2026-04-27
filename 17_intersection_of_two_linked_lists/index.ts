class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function getIntersectionNode(
  headA: ListNode | null,
  headB: ListNode | null,
): ListNode | null {
  const count = (head: ListNode | null): number => {
    let count = 0;
    for (let curr = head; curr != null; curr = curr?.next ?? null) {
      count++;
    }
    return count;
  };

  const countA = count(headA);
  const countB = count(headB);
  const diff = countA - countB;
  const waitStep = Math.abs(diff);

  let longerNode = diff >= 0 ? headA : headB;
  let shorterNode = !(diff >= 0) ? headA : headB;

  let step = 0;

  while (longerNode || shorterNode) {
    const compare = step >= waitStep;
    if (!compare) {
      step++;
      longerNode = longerNode?.next ?? null;
      continue;
    }

    const isIntersect = longerNode === shorterNode;

    if (isIntersect) {
      return longerNode;
    }

    longerNode = longerNode?.next ?? null;
    shorterNode = shorterNode?.next ?? null;
  }

  return null;
}

const main = () => {
  const intersectNode = new ListNode(8, new ListNode(4, new ListNode(5)));

  const nodeA = new ListNode(4, new ListNode(1, intersectNode));
  const nodeB = new ListNode(
    5,
    new ListNode(6, new ListNode(1, intersectNode)),
  );

  const result = getIntersectionNode(nodeA, nodeB);

  console.log(result);
};

main();

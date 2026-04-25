/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */
class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

function sortedArrayToBST(nums: number[]): TreeNode | null {
  if (nums.length === 0) return null;
  if (nums.length === 1) return new TreeNode(nums[0]);

  const middleIndex = Math.round((nums.length - 1) / 2);
  const val = nums[middleIndex];
  const leftNode = sortedArrayToBST(nums.slice(0, middleIndex));
  const rightNode = sortedArrayToBST(nums.slice(middleIndex + 1));

  return new TreeNode(val, leftNode, rightNode);
}

const main = () => {
  const root = sortedArrayToBST([-10, -3, 0, 5, 9]);
};

main();

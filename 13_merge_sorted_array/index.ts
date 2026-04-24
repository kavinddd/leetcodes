function merge(nums1: number[], m: number, nums2: number[], n: number): void {
  if (n === 0) return;

  for (let i = m; i < nums1.length; i++) {
    nums1[i] = nums2[i - m];
  }
  nums1.sort((a, b) => a - b);
}

function main() {
  const nums1 = [0];
  const nums2 = [1];
  merge(nums1, 0, nums2, 1);
  console.log(nums1);
}

main();

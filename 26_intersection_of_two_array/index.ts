const intersection = (nums1: number[], nums2: number[]): number[] => {
  const seen = new Set<number>(nums1);
  const result: number[] = [];
  nums2.forEach((num) => {
    if (seen.has(num)) {
      result.push(num);
      seen.delete(num);
    }
  });
  return result;
};

const main = () => {
  console.log(intersection([1, 2, 2, 1], [2, 2]));
  console.log(intersection([4, 9, 5], [9, 4, 9, 8, 4]));
};

main();

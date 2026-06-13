const intersection_of_two_array_2 = (
  nums1: number[],
  nums2: number[],
): number[] => {
  const count = (nums: readonly number[]): Map<number, number> => {
    return nums.reduce<Map<number, number>>((acc, num) => {
      acc.set(num, (acc.get(num) ?? 0) + 1);
      return acc;
    }, new Map());
  };
  const countNums1 = count(nums1);
  const countNums2 = count(nums2);

  const result: number[] = [];
  for (const [num, count1] of countNums1) {
    const count2 = countNums2.get(num) ?? 0;
    if (count2 !== 0 && count1 !== 0) {
      const length = count1 > count2 ? count2 : count1;
      const intersection = Array.from({ length }).fill(num) as number[];
      result.push(...intersection);
    }
  }

  return result;
};

const main = () => {
  console.log(intersection_of_two_array_2([1, 2, 2, 1], [2, 2]));
  console.log(intersection_of_two_array_2([4, 9, 5], [9, 4, 9, 8, 4]));
};

main();

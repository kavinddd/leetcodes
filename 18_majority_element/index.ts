function majorityElement(nums: number[]): number {
  const countMap: Record<number, number> = {};

  for (let num of nums) {
    countMap[num] = (countMap[num] ?? 0) + 1;
  }

  let maxCount = 0;
  let majority;
  for (let [num, count] of Object.entries(countMap)) {
    if (count > nums.length / 2) {
      return Number(num);
    }

    if (count > maxCount) {
      maxCount = count;
      majority = num;
    }
  }

  return majority;
}

function main() {
  const test1 = majorityElement([3, 2, 3]);
  console.log(test1);
  const test2 = majorityElement([2, 2, 1, 1, 1, 2, 2]);
  console.log(test2);
}

main();

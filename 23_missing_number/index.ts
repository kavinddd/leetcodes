// [0, 8] =>
const missingNumber = (nums: number[]): number => {
  const sumRange = (nums: number[]): number => {
    // derived from (a+b)(b-a+1)/2 given a = 0
    const b = nums.length;
    return (b * (b + 1)) / 2;
  };
  const expectedSumRange = sumRange(nums);
  const actualSumRange = nums.reduce((acc, curr) => acc + curr, 0);
  return expectedSumRange - actualSumRange;
};

const main = () => {
  console.log(missingNumber([0, 1, 2, 3, 4, 6]));
};

main();

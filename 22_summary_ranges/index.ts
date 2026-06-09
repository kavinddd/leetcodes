const summaryRanges = (nums: number[]): string[] => {
  const formatRanges = (nums: readonly number[]): string => {
    if (nums.length === 1) return `${nums[0]}`;
    return `${nums[0]}->${nums[nums.length - 1]}`;
  };

  const result = [];
  const temp = [];

  for (let i = 0; i <= nums.length; i++) {
    const currNum = nums[i];
    const prevNum = nums[i - 1];

    if (prevNum !== undefined && currNum - prevNum !== 1) {
      result.push(formatRanges(temp));
      temp.length = 0;
    }
    temp.push(currNum);
  }

  return result;
};

(() => {
  console.log(summaryRanges([0, 1, 2, 4, 5, 7]));
})();

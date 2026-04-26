const singleNumber = (nums: number[]): number => {
  let temp = nums[0];
  for (let num of nums.slice(1)) {
    temp = temp ^ num;
  }
  return temp;
};

const main = () => {
  console.log(singleNumber([2, 3, 3, 1, 2]));
};

main();

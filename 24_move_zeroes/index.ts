const moveZeroes = (nums: number[]): void => {
  let write = 0;
  let read = 0;

  while (read < nums.length) {
    if (nums[read] !== 0) {
      const temp = nums[write];
      nums[write] = nums[read];
      nums[read] = temp;
      write++;
    }
    read++;
  }
};

const main = () => {
  const test1 = [1];
  moveZeroes(test1);
  console.log(test1);

  const test = [0, 0, 1];
  moveZeroes(test);
  console.log(test);

  const test2 = [0, 1, 0, 3, 12];
  moveZeroes(test2);
  console.log(test2);
};

main();

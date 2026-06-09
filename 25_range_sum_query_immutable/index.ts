class NumArray {
  prefix: number[];

  constructor(nums: number[]) {
    let sum = 0;
    const prefix = new Array(nums.length).fill(0);
    for (let i = 0; i < nums.length; i++) {
      sum += nums[i];
      prefix[i] = sum;
    }
    this.prefix = prefix;

    console.log(prefix);
  }

  sumRange(left: number, right: number): number {
    return this.prefix[right] - (left > 0 ? this.prefix[left - 1] : 0);
  }
}

const main = () => {
  const numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
  console.log(numArray.sumRange(0, 2));
  console.log(numArray.sumRange(2, 5));
  console.log(numArray.sumRange(0, 5));
};

main();

const isHappy = (n: number): boolean => {
  const extractToDigits = (n: number): number[] => {
    const result: number[] = [];
    for (let x of String(n)) {
      result.push(Number(x));
    }
    return result;
  };

  const sumSquared = (xs: number[]): number => {
    let sum = 0;
    while (xs.length > 0) {
      sum += (xs.pop() || 0) ** 2;
    }
    return sum;
  };

  const seen = new Set();

  for (
    let sum = sumSquared(extractToDigits(n));
    !seen.has(sum);
    sum = sumSquared(extractToDigits(sum))
  ) {
    // console.log(
    //   `${extractToDigits(sum)} => ${sumSquared(extractToDigits(sum))}`,
    // );
    if (sum === 1) return true;
    seen.add(sum);
  }

  return false;
};

(() => {
  const result = isHappy(2);
  console.log(result);
})();

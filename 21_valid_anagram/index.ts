const isAnagram = (s: string, t: string): boolean => {
  const countChar = (str: string): Record<string, number> => {
    const result = {};
    for (let char of str) {
      result[char] = (result[char] ?? 0) + 1;
    }
    return result;
  };

  const sCharacterCount = countChar(s);
  const tCharacterCount = countChar(t);

  for (let char of Object.values(s)) {
    const sCount = sCharacterCount[char];
    const tCount = tCharacterCount[char];

    if (sCount !== tCount) {
      return false;
    }
  }

  return new Set(Object.values(s)).size === new Set(Object.values(t)).size;
};

(() => {
  console.log(isAnagram("anagram", "maanagr"));
  console.log(isAnagram("evil", "vile"));
  console.log(isAnagram("rat", "car"));
  console.log(isAnagram("a", "ab"));
})();

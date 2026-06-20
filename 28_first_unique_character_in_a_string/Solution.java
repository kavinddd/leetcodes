class Solution {

	public static int firstUniqChar(String s) {

		int[] freq = new int[26];
		for (char c : s.toCharArray()) {
			freq[c - 'a']++;
		}

		for (int i = 0; i < s.length(); i++) {
			char c = s.charAt(i);
			int frequency = freq[c - 'a'];
			if (frequency == 1)
				return i;
		}

		return -1;

	};

	public static void main(String[] args) {

		int test = firstUniqChar("loveleetcode");
		System.out.println(test);

		int test2 = firstUniqChar("aabb");
		System.out.println(test2);

		int test3 = firstUniqChar("tttttx");
		System.out.println(test3);

	}

}

import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;

class Solution {

	public static char findTheDifference(String s, String t) {
		Map<Character, Long> sSet = s.chars().mapToObj(c -> (char) c)
				.collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
		Map<Character, Long> tSet = t.chars().mapToObj(c -> (char) c)
				.collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

		for (int i = 0; i < t.length(); i++) {

			char c = t.charAt(i);
			Long countS = sSet.getOrDefault(c, 0L);
			Long countT = tSet.getOrDefault(c, 0L);

			if (countT != countS) {
				return c;
			}
		}

		throw new RuntimeException("Two strings are identical");

	}

	public static void main(String[] args) {
		System.out.println(findTheDifference("abcd", "abcde"));
		System.out.println(findTheDifference("", "y"));
		System.out.println(findTheDifference("a", "aa"));

	}

}

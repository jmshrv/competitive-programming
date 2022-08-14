class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        # Note - this function could be easily changed to return the actual
        # substring
        curSubstring = ""
        longestSubstring = ""

        for char in s:
            curSubstring += char

            if curSubstring.count(char) == 2:
                # Cut the string at the first repeated character.
                # For example, "abca" would become "bca"
                curSubstring = curSubstring[curSubstring.index(char) + 1 :]

            if len(curSubstring) > len(longestSubstring):
                longestSubstring = curSubstring

        return len(longestSubstring)


if __name__ == "__main__":
    solution = Solution()

    print(solution.lengthOfLongestSubstring("abcabcbb"))
    print(solution.lengthOfLongestSubstring("bbbbb"))
    print(solution.lengthOfLongestSubstring("pwwkew"))
    print(solution.lengthOfLongestSubstring("dvdf"))
    print(solution.lengthOfLongestSubstring("cdd"))

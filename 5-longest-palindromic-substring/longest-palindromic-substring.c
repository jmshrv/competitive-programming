#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int isPalindrome(char *s)
{
    int len = strlen(s);

    if (len == 1)
        return 1;

    int j = len - 1;

    for (int i = 0; i < len; i++)
    {
        if (i == j)
            return 1;

        char char_i = s[i];
        char char_j = s[j];

        if (char_i != char_j)
            return 0;

        j--;
    }

    return 1;
}

// naive solution - bruteforce. Completes the 1000 C one in about 1ms
// char *longestPalindrome(char *s)
// {
//     int len = strlen(s);

//     if (len == 1)
//         return s;

//     char *longest_palindrome = strdup("");

//     for (int i = 0; i < len; i++)
//     {
//         for (int j = len - 1; j != 0; j--)
//         {

//             char char_i = s[i];
//             char char_j = s[j];

//             if (char_i == char_j)
//             {
//                 char *palindrome = strndup(s + i, j - i + 1);

//                 if (strlen(palindrome) <= strlen(longest_palindrome) || !isPalindrome(palindrome))
//                 {
//                     free(palindrome);
//                     continue;
//                 }

//                 if (strlen(palindrome) > strlen(longest_palindrome))
//                 {
//                     if (longest_palindrome != NULL)
//                         free(longest_palindrome);

//                     longest_palindrome = palindrome;

//                     if (strlen(longest_palindrome) >= strlen(s) / 2)
//                         return longest_palindrome;
//                 }
//                 else
//                     free(palindrome);
//             }
//         }
//     }

//     return longest_palindrome;
// }



int main()
{
    char *ans1 = longestPalindrome("ilikeracecars");
    printf("%s\n", ans1);
    free(ans1);
    char *ans2 = longestPalindrome("babad");
    printf("%s\n", ans2);
    free(ans2);
    char *ans3 = longestPalindrome("cbbd");
    printf("%s\n", ans3);
    free(ans3);
    char *ans4 = longestPalindrome("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    printf("%s\n", ans4);
    free(ans4);
}
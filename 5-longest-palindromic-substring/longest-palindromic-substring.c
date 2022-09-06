#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define max(a, b) \
    ({ __typeof__ (a) _a = (a); \
       __typeof__ (b) _b = (b); \
     _a > _b ? _a : _b; })

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

// Manacher's algorithm - O(n), but I didn't really understand the
// implementation.
// /// Generates a string with | in between each character. bogusS must be
// /// allocated beforehand (malloc(2 * strlen(s) + 1 + 1))
// void generateBogusString(char *bogusS, char *s)
// {
//     char *bogus_pointer = bogusS;

//     for (int i = 0; i < strlen(s); i++)
//     {
//         *bogus_pointer = '|';
//         bogus_pointer++;
//         *bogus_pointer = s[i];
//         bogus_pointer++;
//     }

//     *bogus_pointer = '|';
//     *bogus_pointer++;
//     *bogus_pointer = '\0';
// }

// char *longestPalindrome(char *s)
// {
//     int bogusS_len = 2 * strlen(s) + 1 + 1;

//     char *bogusS = calloc(bogusS_len, sizeof(char));
//     generateBogusString(bogusS, s);

//     int *palindrome_radii = calloc(bogusS_len - 1, sizeof(int));

//     int center = 0;
//     int radius = 0;

//     while (center < strlen(bogusS))
//     {
//         while (center - radius + 1 >= 0 && center + radius + 1 < strlen(bogusS) && bogusS[center - radius + 1] == bogusS[center + radius + 1])
//             radius++;

//         palindrome_radii[center] = radius;

//         int old_center = center;
//         int old_radius = radius;

//         center++;
//         radius = 0;

//         while (center <= old_center + old_radius)
//         {
//             int mirrored_center = old_center - center - old_center;
//             int max_mirrored_radius = old_center + old_radius - center;

//             if (palindrome_radii[mirrored_center] < max_mirrored_radius)
//             {
//                 palindrome_radii[center] = palindrome_radii[mirrored_center];
//                 center++;
//             }
//             else if (palindrome_radii[mirrored_center] > max_mirrored_radius)
//             {
//                 palindrome_radii[center] = max_mirrored_radius;
//                 center++;
//             }
//             else
//             {
//                 radius = max_mirrored_radius;
//                 break;
//             }
//         }
//     }

//     putchar('[');
//     for (int i = 0; i < bogusS_len; i++)
//     {
//         printf("%d,", palindrome_radii[i]);
//     }
//     putchar(']');
//     putchar('\n');
//     return bogusS;
// }

// Expand around center - O(n²)
int expandAroundCenter(char *s, int left, int right)
{
    int L = left;
    int R = right;

    while (L >= 0 && R < strlen(s) && s[L] == s[R])
    {
        L--;
        R++;
    }

    return R - L - 1;
}

char *longestPalindrome(char *s)
{
    int start = 0;
    int end = 0;

    for (int i = 0; i < strlen(s); i++)
    {
        int len1 = expandAroundCenter(s, i, i);
        int len2 = expandAroundCenter(s, i, i + 1);
        int len = max(len1, len2);

        if (len > end - start)
        {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    return strndup(s + start, end - start + 1);
}

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
    // char *ans4 = longestPalindrome("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    // printf("%s\n", ans4);
    // free(ans4);
}
#include <iostream>
#include <vector>

int main()
{
    size_t column_length;
    std::cin >> column_length;

    std::vector<std::vector<int>> pathway = {{}, {}};

    // This isn't really necessary but the input gives us the size so we may as
    // well avoid reallocations
    for (auto &column : pathway)
    {
        column.reserve(column_length);
    }
}
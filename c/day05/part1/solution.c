#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

struct NumbersWithSize
{
    long *p;
    size_t size;
};

struct ConversionMap
{
    long destination_range_start;
    long source_range_start;
    long range_length;
};

struct NumbersWithSize extract_seeds(char *seed_str)
{
    int seed_size = 0;
    long *seeds = calloc(seed_size, sizeof(long));
    long number = 0;
    bool has_number = false;

    for (int i = 0; i < strlen(seed_str); i++)
    {
        if (isdigit(seed_str[i]))
        {
            has_number = true;
            number = number * 10 + (seed_str[i] - '0');
        }
        else if (has_number)
        {
            seed_size++;
            seeds = realloc(seeds, seed_size * sizeof(long));
            seeds[seed_size - 1] = number;
            number = 0;
            has_number = false;
        }
    }

    struct NumbersWithSize seeds_with_size;
    seeds_with_size.size = seed_size;
    seeds_with_size.p = seeds;

    return seeds_with_size;
}

struct ConversionMap extract_conversion_map(char *str)
{
    struct ConversionMap conversion_map;

    int number_column = 0;
    long number = 0;
    bool has_number = false;

    for (int i = 0; i < strlen(str); i++)
    {
        if (isdigit(str[i]))
        {
            has_number = true;
            number = number * 10 + (str[i] - '0');
        }
        else if (has_number)
        {
            switch (number_column)
            {
            case 0:
                conversion_map.destination_range_start = number;
                break;
            case 1:
                conversion_map.source_range_start = number;
                break;
            case 2:
                conversion_map.range_length = number;
                break;
            }

            number_column++;
            number = 0;
            has_number = false;
        }
    }

    if (has_number)
    {
        // Last line doesn't have a line break
        // Which causes it to not be handled in the loop
        // Just add the last number to the range
        conversion_map.range_length = number;
    }

    return conversion_map;
}

long convert_number(long number, struct ConversionMap *conversion_maps, int conversion_map_size)
{
    for (int i = 0; i < conversion_map_size; i++)
    {
        struct ConversionMap conversion_map = conversion_maps[i];
        if (conversion_map.source_range_start <= number && conversion_map.source_range_start + conversion_map.range_length > number)
        {
            return number + conversion_map.destination_range_start - conversion_map.source_range_start;
        }
    }

    return number;
}

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        fprintf(stderr, "Missing input file argument\n");
        return EXIT_FAILURE;
    }
    char *file_name = argv[1];

    FILE *file = fopen(file_name, "r");
    if (file == NULL)
    {
        fprintf(stderr, "Could not open file with filename: %s\n", file_name);
        return EXIT_FAILURE;
    }

    char *contents = NULL;
    size_t length = 0;
    if (getline(&contents, &length, file) == -1)
    {
        fprintf(stderr, "Missing seeds header in file with filename: %s\n", file_name);
        return EXIT_FAILURE;
    }

    struct NumbersWithSize numbers_with_size = extract_seeds(contents);

    int conversion_map_size = 0;
    struct ConversionMap *conversion_maps = calloc(conversion_map_size, sizeof(struct ConversionMap));
    long number = 0;
    bool has_number = false;

    while (getline(&contents, &length, file) != -1)
    {
        if (isdigit(contents[0]))
        {
            struct ConversionMap conversion_map = extract_conversion_map(contents);
            conversion_map_size++;
            conversion_maps = realloc(conversion_maps, conversion_map_size * sizeof(struct ConversionMap));
            conversion_maps[conversion_map_size - 1] = conversion_map;
        }
        else if (conversion_map_size > 0)
        {
            for (int i = 0; i < numbers_with_size.size; i++)
            {
                numbers_with_size.p[i] = convert_number(numbers_with_size.p[i], conversion_maps, conversion_map_size);
            }
            conversion_map_size = 0;
            conversion_maps = realloc(conversion_maps, conversion_map_size * sizeof(struct ConversionMap));
        }
    }

    for (int i = 0; i < numbers_with_size.size; i++)
    {
        numbers_with_size.p[i] = convert_number(numbers_with_size.p[i], conversion_maps, conversion_map_size);
    }

    long min_location = numbers_with_size.p[0];
    for (int i = 1; i < numbers_with_size.size; i++)
    {
        long location = numbers_with_size.p[i];
        if (min_location > location)
        {
            min_location = location;
        }
    }

    printf("Result: %lu\n", min_location);

    fclose(file);
    free(contents);
    free(numbers_with_size.p);
    free(conversion_maps);

    return EXIT_SUCCESS;
}

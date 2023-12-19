#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

struct Pair
{
    long range_start;
    long range_length;
};

struct PairsWithSize
{
    struct Pair *pairs;
    size_t size;
};

struct ConversionMap
{
    long destination_range_start;
    long source_range_start;
    long range_length;
};

struct PairsWithSize extract_seed_pair(char *seed_str)
{
    int seed_size = 0;
    struct Pair *seed_pairs = calloc(seed_size, sizeof(struct Pair));
    struct Pair seed_pair;
    long number = 0;
    bool has_number = false;
    long current_pair_part = 1;

    for (int i = 0; i < strlen(seed_str); i++)
    {
        if (isdigit(seed_str[i]))
        {
            has_number = true;
            number = number * 10 + (seed_str[i] - '0');
        }
        else if (has_number)
        {
            if (current_pair_part == 1)
            {
                seed_pair.range_start = number;
                current_pair_part = 2;
            }
            else if (current_pair_part == 2)
            {
                seed_pair.range_length = number;
                current_pair_part = 1;

                seed_size++;
                seed_pairs = realloc(seed_pairs, seed_size * sizeof(struct Pair));

                seed_pairs[seed_size - 1] = seed_pair;
            }

            number = 0;
            has_number = false;
        }
    }

    struct PairsWithSize seeds_with_size;
    seeds_with_size.size = seed_size;
    seeds_with_size.pairs = seed_pairs;

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

void process_pairs(struct PairsWithSize *pairs_with_size, struct ConversionMap *conversion_maps, int conversion_map_size)
{
    for (int i = 0; i < pairs_with_size->size; i++)
    {
        struct Pair pair = pairs_with_size->pairs[i];

        for (int i = 0; i < conversion_map_size; i++)
        {
            struct ConversionMap conversion_map = conversion_maps[i];

            bool has_pairs_between = conversion_map.source_range_start < pair.range_start + pair.range_length && conversion_map.source_range_start + conversion_map.range_length > pair.range_start;
            bool is_pairs_above = conversion_map.source_range_start + conversion_map.range_length < pair.range_start + pair.range_length;
            bool is_pairs_below = conversion_map.source_range_start > pair.range_start;

            if (!has_pairs_between)
            {
                continue;
            }

            if (is_pairs_above)
            {
                long common_range = conversion_map.source_range_start + conversion_map.range_length - pair.range_start;
                long missing_range = pair.range_length - common_range;

                pairs_with_size->size++;
                pairs_with_size->pairs = realloc(pairs_with_size->pairs, pairs_with_size->size * sizeof(struct Pair));

                struct Pair new_pair;
                new_pair.range_start = pair.range_start + common_range;
                new_pair.range_length = missing_range;
                pairs_with_size->pairs[pairs_with_size->size - 1] = new_pair;

                pair.range_length = common_range;
            }

            if (is_pairs_below)
            {
                long common_range = pair.range_start + pair.range_length - conversion_map.source_range_start;
                long missing_range = pair.range_length - common_range;

                pairs_with_size->size++;
                pairs_with_size->pairs = realloc(pairs_with_size->pairs, pairs_with_size->size * sizeof(struct Pair));

                struct Pair new_pair;
                new_pair.range_start = pair.range_start;
                new_pair.range_length = missing_range;
                pairs_with_size->pairs[pairs_with_size->size - 1] = new_pair;

                pair.range_start += missing_range;
                pair.range_length = common_range;
            }

            pair.range_start = pair.range_start + conversion_map.destination_range_start - conversion_map.source_range_start;
            break;
        }

        pairs_with_size->pairs[i] = pair;
    }
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

    struct PairsWithSize pairs_with_size = extract_seed_pair(contents);

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
            process_pairs(&pairs_with_size, conversion_maps, conversion_map_size);
            conversion_map_size = 0;
            conversion_maps = realloc(conversion_maps, conversion_map_size * sizeof(struct ConversionMap));
        }
    }
    process_pairs(&pairs_with_size, conversion_maps, conversion_map_size);

    long min_location = pairs_with_size.pairs[0].range_start;
    for (int i = 1; i < pairs_with_size.size; i++)
    {
        long location = pairs_with_size.pairs[i].range_start;
        if (min_location > location)
        {
            min_location = location;
        }
    }

    printf("Result: %lu\n", min_location);

    fclose(file);
    free(contents);
    free(pairs_with_size.pairs);
    free(conversion_maps);

    return EXIT_SUCCESS;
}

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv)
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: %s <input.txt>\n", argv[0]);
        exit(-1);
    }
    
    FILE * input = fopen(argv[1], "r");
    if (input == NULL)
    {
        fprintf(stderr, "%s: file not found\n", argv[1]);
        exit(-1);
    }

    int * pelf_calories = (int *) calloc(1, sizeof(int));
    if (pelf_calories == NULL)
    {
        fprintf(stderr, "Cannot alloc calorie array\n");
        exit(-1);
    }
   
    char * curr_line;
    size_t linesize;
    size_t nelfs = 1;
    while (getline(&curr_line, &linesize, input) != -1)
    {
        if (curr_line[0] == '\n')
        {
            pelf_calories = realloc((void *) pelf_calories, (++nelfs) * sizeof(int));
            pelf_calories[nelfs-1] = 0;
        }
        else
        {
            int tmp_cals = 0;
            sscanf(curr_line, "%d", &tmp_cals);
            pelf_calories[nelfs-1] += tmp_cals;
        }
    }

    int max_cals = 0;
    int max_2 = 0;
    int max_3 = 0;
    for(int i = 0; i < nelfs; i++)
    {
        if (pelf_calories[i] > max_cals)
        {
            max_3 = max_2;
            max_2 = max_cals;
            max_cals = pelf_calories[i];
        }
    }

    printf("%d max calories\n", max_cals);
    printf("%d 2nd max calories\n", max_2);
    printf("%d 3rd max calories\n", max_3);
    printf("total: %d\n", max_cals + max_2 + max_3);
    free(pelf_calories);

    return 0;
}

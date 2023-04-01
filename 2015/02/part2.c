#include <stdio.h>

/**
 * Sort the array in place, uses
 * insertion sort algorithm.
 * Smallest number will be first.
 */
void sort_array(int *input_array, int length) {
  int i, j, key;
  for (i = 1; i < length; i++) {
    key = input_array[i];
    j = i - 1;
    while (j > -1 && input_array[j] > key) {
      input_array[j + 1] = input_array[j];
      j -= 1;
    }
    input_array[j + 1] = key;
  }
}

int get_ribbon_length(int *sides) {
  sort_array(sides, 3);
  printf("[%d, %d, %d]\n", sides[0], sides[1], sides[2]);

  return 2 * (sides[0] + sides[1])         /* perimeter */
         + sides[0] * sides[1] * sides[2]; /* Bow length */
}

int main() {
  FILE *f = fopen("input.txt", "r");
  int buffer_size = 11;
  char buffer[buffer_size];

  int sides[3];
  int required_ribbon_length = 0;
  while (fgets(buffer, buffer_size, f) != NULL) {
    sscanf(buffer, "%dx%dx%d\n", sides, &sides[1], &sides[2]);
    required_ribbon_length += get_ribbon_length(sides);
  }
  printf("Ribbon length needed: %d\n", required_ribbon_length);

  return 0;
}

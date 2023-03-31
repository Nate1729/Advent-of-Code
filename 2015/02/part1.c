#include <stdio.h>
#include <stdlib.h>

typedef unsigned long ul;

/* NOTES
 *
 * Looks like the max size is two
 * digits for numbers so each line can only be
 * 2*3 + 3 + 1 = 10 characters long
 */

ul calculate_surface_area(int length, int width, int height) {
  int face1 = length * width;
  int face2 = length * height;
  int face3 = height * width;

  ul surface_area = (ul)2 * face1 + 2 * face2 + 2 * face3;
  if (face1 < face2) {
    if (face1 < face3) {
      return surface_area + (ul)face1;
    } else {
      return surface_area + (ul)face3;
    }
  } else {
    if (face2 < face3) {
      return surface_area + (ul)face2;
    } else {
      return surface_area + (ul)face3;
    }
  }
}

int main(int argc, char **argv) {
  FILE *f;
  if (argc == 2) {
    f = fopen(argv[1], "r");
  } else {
    f = fopen("input.txt", "r");
  }

  char *buffer = malloc(sizeof(char) * 11);
  int length, width, height;
  ul total = 0;
  while (fgets(buffer, 11, f)) {
    sscanf(buffer, "%dx%dx%d\n", &length, &width, &height);

    total += calculate_surface_area(length, width, height);
  }

  printf("Total: %lu\n", total);
  free(buffer);
  fclose(f);
  return 0;
}

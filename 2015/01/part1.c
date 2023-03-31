#include <stdio.h>

int main(int argc, char **argv) {
  FILE *f;
  f = fopen("input.txt", "r");

  char c;
  int floor = 0;
  c = getc(f);
  while (c != EOF)
  {
    if (c == '(')
    {
      floor += 1;
    } 
    else if (c == ')')
    {
      floor -= 1;
    }
    c = getc(f);
  }

  printf("result floor: %d\n", floor);
    return 0;
}

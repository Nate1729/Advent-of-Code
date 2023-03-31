#include <stdio.h>

int main()
{
  FILE *f = fopen("input.txt", "r");

  int counter = 1;
  int floor = 0;
  char c = getc(f);
  while (c != EOF)
  {
    if (c == '(')
      floor += 1;
    if (c == ')')
      floor -= 1;

    if (floor == -1)
    {
      printf("First entered basement at character position: %d\n", counter);
    }
    c = getc(f);
    counter +=1 ;
  }

  return 0;
}

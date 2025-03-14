#include <stdio.h>

int add(int a, int b) {
  return a+b;
}

int main() {
  int a = 4;
  int b = 9;
  printf("The result of %d and %d is %d\n", a, b, add(a, b));

  return 0;
}


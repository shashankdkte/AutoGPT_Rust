#include <stddef.h>

int sumArray(int* arr, size_t length)
{
  int sum = 0;
  for (size_t i = 0; i < length;i++)
  {
    sum += arr[i];
  }
  return sum;
}
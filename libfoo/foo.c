#include <stdio.h>

void pass_a_float(float value) { printf("Float received: %f\n", value); }

void pass_a_string(const char *value) {
  printf("String received: %s\n", value);
}

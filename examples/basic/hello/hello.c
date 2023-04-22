#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  char *buf = "Hello!\n";
  write(1, buf, strlen(buf));
  int f1 = open("testfile.txt", O_CREAT | O_WRONLY | O_TRUNC);
  write(f1, "Another print statement", 23);
  close(f1);
  FILE *f2 = fopen("AnotherTest.txt", "w+");
  int result = fputs("Testing fputs tracing here", f2);
  if (result < 0) {
    printf("Error!\n");
  }
  fclose(f2);
  return 0;
}

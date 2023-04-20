#include <fcntl.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  char *buf = "Hello!\n";
  write(1, buf, strlen(buf));
  int f1 = open("testfile.txt", O_CREAT | O_WRONLY);
  write(f1, "Another print statement", 23);
  close(f1);
  return 0;
}

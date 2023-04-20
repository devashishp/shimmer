EXECBIN  = hello

SOURCES  = $(wildcard *.c)
OBJECTS  = $(SOURCES:%.c=%.o)

CC       = clang-15
CFLAGS   = -Wall -Wpedantic -Werror -Wextra -gdwarf-4

.PHONY: all clean

all: $(EXECBIN)

$(EXECBIN): $(OBJECTS)
	$(CC) -o $@ $^

%.o : %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f $(EXECBIN) $(OBJECTS)

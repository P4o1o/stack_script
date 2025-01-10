CC = gcc
CFLAGS = -O3 -Wall -pedantic -std=c18
DFLAGS = -g
SRCDIR = src
BINDIR = bin
SRCFILES = $(wildcard $(SRCDIR)/*.c)
OBJFILES = $(patsubst $(SRCDIR)/%.c,$(BINDIR)/%.o,$(SRCFILES))

all: sscript

sscript : $(OBJFILES)
	$(CC) $(CFLAGS) $(DFLAGS) -o sscript $(OBJFILES) -lm -fopenmp

$(BINDIR)/%.o: $(SRCDIR)/%.c | $(BINDIR)
	$(CC) $(CFLAGS) $(DFLAGS) -c $< -o $@ -lm -fopenmp

$(BINDIR):
	mkdir $(BINDIR)


.PHONY: clean
clean:
	rm -rf $(BINDIR)/*.o
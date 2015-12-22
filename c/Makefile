CPP=clang-17

.PHONY: clean test

# Build and run all tests.
test: test.out
	../test.sh test.out solution.c

# Clean outputs.
clean:
	rm -rf *.o test.out

# Build tests.
test.out: *.c *.h
	$(CPP) -O3 -o test.out $$(ls *.c | grep -v solution_blank.c) -lm

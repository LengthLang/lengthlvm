#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

#include <length.h>

typedef struct Stack {
	int64_t* contents;
	size_t pointer;
	size_t cap;
} Stack;

Stack* initStack() {
	Stack* stack;
	stack = (Stack*)malloc(sizeof(Stack));
	if (stack == NULL) {
		fprintf(stderr, "Not enough memory for stack struct");
		exit(1);
	}

	stack->contents = (uint64_t*)malloc(sizeof(int64_t) * 16);
	if (stack->contents == NULL) {
		fprintf(stderr, "Not enough memory for stack contents");
		exit(1);
	}
	stack->cap = 16;
	stack->pointer = 0;
}

void freeStack(Stack* stack) {
	free(stack->contents);
	free(stack);
}

void push(Stack* stack, int64_t value) {
	if (stack->pointer == stack->cap) {
		stack->contents = realloc(stack->contents, (stack->cap * 2) * sizeof(int64_t));
		stack->cap *= 2;
		if (stack->contents == NULL) {
			fprintf(stderr, "Not enough memory for stack contents (Stack overflow)");
			exit(1);
		}
	}
	stack->contents[stack->pointer] = value;
	stack->pointer++;
}

int64_t pop(Stack* stack) {
	stack->pointer--;
	if (stack->pointer < 0) {
		fprintf(stderr, "Stack underflow");
		exit(1);
	}
	return stack->contents[stack->pointer];
}


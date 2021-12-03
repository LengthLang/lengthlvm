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

	stack->contents = (int64_t*)malloc(sizeof(int64_t) * 16);
	if (stack->contents == NULL) {
		fprintf(stderr, "Not enough memory for stack contents");
		exit(1);
	}
	stack->cap = 16;
	stack->pointer = 0;

	return stack;
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

void lengthInp(Stack* stack) {
	push(stack, getc(stdin));
}

void lengthAdd(Stack* stack) {
	int64_t a = pop(stack);
	int64_t b = pop(stack);

	push(stack, a + b);
}

void lengthSub(Stack* stack) {
	int64_t a = pop(stack);
	int64_t b = pop(stack);

	push(stack, b - a);
}

void lengthDup(Stack* stack) {
	int64_t a = pop(stack);

	push(stack, a);
	push(stack, a);
}

void lengthOutn(Stack* stack) {
	printf("%ld", pop(stack));
}

void lengthOuta(Stack* stack) {
	printf("%c", (char)pop(stack));
}

void lengthRol(Stack* stack) {
	int64_t* oldBuffer = stack->contents;
	stack->contents = malloc(stack->cap * sizeof(int64_t));
	if (stack->contents == NULL) {
		fprintf(stderr, "Not enough memory for stack contents (Stack overflow)");
		exit(1);
	}

	size_t oldPointer = stack->pointer;

	stack->pointer = 0;
	push(stack, oldBuffer[oldPointer - 1]);
	for (int i = 0; i < oldPointer - 1; i++) {
		push(stack, oldBuffer[i]);
	}

	free(oldBuffer);
}


void lengthSwap(Stack* stack) {
	int64_t a = pop(stack);
	int64_t b = pop(stack);

	push(stack, a);
	push(stack, b);
}

void lengthMul(Stack* stack) {
	int64_t a = pop(stack);
	int64_t b = pop(stack);

	push(stack, a * b);
}

void lengthDiv(Stack* stack) {
	int64_t a = pop(stack);
	int64_t b = pop(stack);

	push(stack, b / a);
}

void lengthRor(Stack* stack) {
	int64_t* oldBuffer = stack->contents;
	stack->contents = malloc(stack->cap * sizeof(int64_t));
	if (stack->contents == NULL) {
		fprintf(stderr, "Not enough memory for stack contents (Stack overflow)");
		exit(1);
	}

	size_t oldPointer = stack->pointer;
	stack->pointer = 0;
	for (size_t i = 1; i < oldPointer; i++) {
		push(stack, oldBuffer[i]);
	}

	for (size_t i = 0; i < oldPointer; i++) {
	}

	push(stack, oldBuffer[0]);
	free(oldBuffer);
}
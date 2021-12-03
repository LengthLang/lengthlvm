#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN

#include "doctest.h"
#include <length.h>

TEST_SUITE("Stack") {
	TEST_CASE("Sanity check: create and free stack") {
		Stack* stack = initStack();

		freeStack(stack);
	}

	TEST_CASE("Push and pop") {
		Stack* stack = initStack();

		push(stack, 42);
		push(stack, 123);

		CHECK_EQ(pop(stack), 123);
		CHECK_EQ(pop(stack), 42);

		freeStack(stack);
	}

	TEST_CASE("Push over cap") {
		Stack* stack = initStack();

		for (int i = 0; i < 255; i++) {
			push(stack, 42);
		}

		for (int i = 0; i < 255; i++) {
			CHECK_EQ(pop(stack), 42);
		}

		freeStack(stack);
	}

	TEST_CASE("Arithmetic operations") {
		Stack* stack = initStack();

		push(stack, 6);
		push(stack, 7);
		lengthMul(stack);
		CHECK_EQ(pop(stack), 42);

		push(stack, 3);
		push(stack, 6);
		lengthAdd(stack);
		CHECK_EQ(pop(stack), 9);

		push(stack, 100);
		push(stack, 2);
		lengthDiv(stack);
		CHECK_EQ(pop(stack), 50);

		push(stack, 44);
		push(stack, 2);
		lengthSub(stack);
		CHECK_EQ(pop(stack), 42);

		freeStack(stack);
	}

	TEST_CASE("Stack operations") {
		Stack* stack = initStack();

		push(stack, 5);
		push(stack, 6);
		push(stack, 7);
		lengthRol(stack);
		CHECK_EQ(pop(stack), 6);
		CHECK_EQ(pop(stack), 5);
		CHECK_EQ(pop(stack), 7);

		// freeStack(stack);
		// stack = initStack();

		push(stack, 5);
		push(stack, 6);
		push(stack, 7);
		lengthRor(stack);
		CHECK_EQ(pop(stack), 5);
		CHECK_EQ(pop(stack), 7);
		CHECK_EQ(pop(stack), 6);

		push(stack, 42);
		push(stack, 123);
		lengthSwap(stack);
		CHECK_EQ(pop(stack), 42);
		CHECK_EQ(pop(stack), 123);

		freeStack(stack);
	}
}
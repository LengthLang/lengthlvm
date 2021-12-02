#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN

#include "doctest.h"
#include <length.h>

TEST_SUITE("Stack") {
	TEST_CASE("create and free stack") {
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
}
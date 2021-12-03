#include <stdint.h>

#ifndef length_h
#define length_h

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Stack Stack;

Stack* initStack();
void freeStack(Stack* stack);
void push(Stack* stack, int64_t value);
int64_t pop(Stack* stack);

void lengthInp(Stack* stack);
void lengthAdd(Stack* stack);
void lengthSub(Stack* stack);
void lengthDup(Stack* stack);
void lengthOutn(Stack* stack);
void lengthOuta(Stack* stack);
void lengthRol(Stack* stack);
void lengthSwap(Stack* stack);
void lengthMul(Stack* stack);
void lengthDiv(Stack* stack);
void lengthRor(Stack* stack);

#ifdef __cplusplus
}
#endif

#endif
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Stack Stack;

Stack* initStack();
void freeStack(Stack* stack);
void push(Stack* stack, int64_t value);
int64_t pop(Stack* stack);

#ifdef __cplusplus
}
#endif
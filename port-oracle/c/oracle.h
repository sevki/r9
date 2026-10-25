/*
 * Stand-in for Harvey's "u.h", "dat.h" and friends: just the types and
 * helpers the oracle sources use, so they build unmodified with a hosted C
 * compiler.
 */
#include <stdint.h>

typedef uint8_t u8;
typedef int32_t i32;
typedef uint32_t Rune;
typedef uint64_t u64;

#define nil ((void *)0)

/* From port/portdat.h. */
typedef struct Watermark Watermark;
struct Watermark {
	int highwater;
	int curr;
	int max;
	int hitmax; /* count: how many times hit max? */
	char *name;
};

char *seprint(char *buf, char *ebuf, char *fmt, ...);
void *memset(void *s, int c, unsigned long n);

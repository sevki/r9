/*
 * Shadows Harvey's amd64 u.h when running c2rust. c2rust cannot translate
 * __builtin_return_address, so every function that calls getcallerpc()
 * would be silently dropped from the output. getcallerpc() is only used to
 * record who took a lock, for diagnostics, so the translation uses 0.
 */
#include_next "u.h"

#undef getcallerpc
#define getcallerpc() ((usize)0)

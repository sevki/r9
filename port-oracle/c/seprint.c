/* Hosted-libc stand-in for Plan 9's seprint(). */
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

char *
seprint(char *buf, char *ebuf, char *fmt, ...)
{
	va_list ap;
	int n;

	if(buf >= ebuf)
		return buf;
	va_start(ap, fmt);
	n = vsnprintf(buf, ebuf - buf, fmt, ap);
	va_end(ap);
	if(n >= ebuf - buf)
		return ebuf - 1;
	return buf + n;
}

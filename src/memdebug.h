#ifndef MEMDEBUG_H_INCLUDED
#define MEMDEBUG_H_INCLUDED
#include <stddef.h>

struct memblock {
	void* addr;
	char* file;
	size_t size;
	size_t line;
	struct memblock* next;
};

void* debug_malloc(size_t size, char* file, size_t line);
void* debug_calloc(size_t nmemb, size_t size, char* file, size_t line);
void* debug_realloc(void* ptr, size_t size, char* file, size_t line);
void debug_free(void* ptr, char* file, size_t line);
void print_allocated_mem(void);


#define malloc(size) debug_malloc(size, __FILE__, __LINE__)
#define free(ptr) debug_free(ptr, __FILE__, __LINE__)
#define calloc(nmemb, size) debug_calloc(nmemb, size, __FILE__, __LINE__)
#define realloc(ptr, size) debug_realloc(ptr, size, __FILE__, __LINE__)

#endif 

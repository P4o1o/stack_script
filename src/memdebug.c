#include "memdebug.h"

#undef malloc
#undef calloc
#undef realloc
#undef free

#include <stdio.h>
#include <stdlib.h>

struct memblock* allocated_mem = NULL;

inline int new_memblock(void *addr, size_t size, char* file, int line){
	struct memblock *res = (struct memblock*) malloc(sizeof(struct memblock));
	if (res == NULL)
		return 0;
	res->addr = addr;
	res->size = size;
	res->file = file;
	res->line = line;
	res->next = allocated_mem;
	allocated_mem = res;
	return 1;
}

inline int remove_memblock(void* addr) {
	struct memblock** actual = &allocated_mem;
	while (*actual != NULL) {
		if ((*actual)->addr == addr) {
			struct memblock* temp = *actual;
			*actual = (*actual)->next;
			free(temp);
			return 1;
		}
		actual = &(*actual)->next;
	}
	return 0;
}

void* debug_malloc(size_t size, char* file, size_t line) {
	void* ptr = malloc(size);
	if (ptr != NULL) {
		if (!new_memblock(ptr, size, file, line)) {
			printf("\nDEBUGGER ERROR!!!\n Can not allocate memory for tracking the pointer created with malloc with addr: %p, defined in file: %s, at line: %zu\n", ptr, file, line);
		}
	}
	return ptr;
}

void* debug_calloc(size_t nmemb, size_t size, char* file, size_t line) {
	void* ptr = calloc(nmemb, size);
	if (ptr != NULL) {
		if (!new_memblock(ptr, size * nmemb, file, line)) {
			printf("\nDEBUGGER ERROR!!!\n Can not allocate memory for tracking the pointer created with calloc with addr: %p, defined in file: %s, at line: %zu\n", ptr, file, line);
		}
	}
	return ptr;
}

void* debug_realloc(void* ptr, size_t size, char* file, size_t line) {
	if (ptr != NULL) {
		if(!remove_memblock(ptr))
			printf("\nERROR!\nAttempted to realloc a pointer that was never allocted in the heap,\naddr: %p, file: %s, line: %zu\n\n", ptr, file, line);
	}
	void* new_ptr = realloc(ptr, size);
	if (new_ptr != NULL) {
		if (!new_memblock(new_ptr, size, file, line)) {
			printf("DEBUGGER ERROR!!!\n Can not allocate memory for tracking the pointer created with realloc with addr: %p, defined in file: %s, at line: %zu\n", new_ptr, file, line);
		}
	}
	return new_ptr;
}

void debug_free(void* ptr, char* file, size_t line) {
	if (ptr == NULL) {
		printf("\nERROR!\nAttempted to free a NULL pointer\nfile: %s, line: %zu\n\n", file, line);
	}else {
		if(! remove_memblock(ptr))
			printf("\nERROR!\nAttempted to free a pointer that was never allocted in the heap\n addr: %p, file: %s, line: %zu\n\n", ptr, file, line);
		free(ptr);
	}
}

void print_allocated_mem(void) {
	struct memblock* block = allocated_mem;
	printf("\nMemory allocated in the heap:\n\naddr\t\t\tsize\t\t\tfile\t\t\t\t\tline");
	if (block == NULL)
		printf("\n\nNONE\n");
	else
		while (block != NULL) {
			printf("\n%p\t%zu\t%s\t%zu\n",
				block->addr, block->size, block->file, block->line);
			block = block->next;
		}
	printf("\n");
}
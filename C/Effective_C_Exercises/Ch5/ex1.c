#include <stdio.h>
//Modify the function from Listing 5-11 to make it clear to the caller
//which file could not be opened.

int do_something(void){
	FILE *file1, *file2;
	object_t *obj;
int ret_val = 0; // Initially assume a successful return value

file1 = fopen("a_file", "w");
if (file1 == NULL){
	ret_val = -1;
	goto FAIL_FILE1;
}

file2 = fopen("another_file", "w");
if (file2 == NULL){
	ret_val = -1;
	goto FAIL_FILE2;
}

obj = malloc(sizeof(object_t));
if (obj == NULL){
	ret_val = -1;
	goto FAIL_OBJ;
}

// Operate on allocated resources


// Clean up everything
free(obj);
FAIL_OBJ:
	printf("The object file could not be opened \n");
	fclose(file2);
FAIL_FILE2:
	printf("The file2 could not be opened \n");
	fclose(file1);
FAIL_FILE1:
	printf("The file1 could not be opened \n");
	return ret_val;


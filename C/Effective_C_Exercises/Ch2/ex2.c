#include <stdio.h>
#include <string.h>

//Declare an array of three pointers to functions
//and invoke the appropriate function based on
//an index value
//passed in as an argument.

void firstFunc(void){
	printf("This is first Pointer to Function");
}

void secondFunc(void){
	printf("This is second Pointer to Function!");

}

void thirdFunc(void){
	printf("THIS IS THIRD POINTER TO FUNCTION!!! HAZAAAAAH!");
}

int main(int argc, char *argv[]){
	if (argc == 2){
		 void (*funcArray[3])(void) = {firstFunc, secondFunc, thirdFunc};

		if (strcmp(argv[1], "1")==0){
			(*funcArray[0])();
		}
		else if (strcmp(argv[1],"2")==0){
			(*funcArray[1])();
		}
		else if (strcmp(argv[1],"3")==0){
			(*funcArray[2])();
		}
		else{
			printf("You gave wrong index");
		}

	return 0;
	}
}


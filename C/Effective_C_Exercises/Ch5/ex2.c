#include <stdio.h>

//Change the find_element function from Listing 5-13
//to return the position of the key in a.
//Don’t forget to return an error indication if the key is not found

size_t find_element(size_t len, int arr[len], int key){
	size_t pos = (size_t)-1;

	// Traver arr and search for key
	for (size_t i = 0; i < len; ++i){
		if (arr[i] == key){
		pos = i
		}
		break; // terminate loop
	}
	return pos;
}


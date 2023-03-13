#include <sys/wait.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>




// Function declarations for builtin shell commands:

int clsh_cd(char ** args);
int clsh_help(char **args);
int clsh_exit(char **args);

char *builtin_str[] = {
	"cd",
	"help",
	"exit"
};

int (*builtin_func[]) (char **) = {
	&clsh_cd,
	&clsh_help,
	&clsh_exit
};

int clsh_num_builtins(){
	return sizeof(builtin_str) / sizeof(char *);
}

// Builtin function implementations.

int clsh_cd(char **args){
	if (args[1] == NULL){
		fprintf(stderr, "clsh: expected argument to \"cd|\n");
	}else {
		if (chdir(args[1]) != 0) {
			perror("clsh");
		}
	}
	return 1;
}

int clsh_help(char **args){
	int i;
	printf("Cloufish's CLSH \n");
	printf("Type program names and arguments, and hit enter.\n");
	printf("The following are built in: \n");

	for ( i = 0; i < clsh_num_builtins(); i++){
		printf(" %s\n", builtin_str[i]);
	}

	printf ("Use the man command for information on other programs.\n");
	return 1;
}

int clsh_exit(char **args){
	return 0;
}

int clsh_launch(char **args){

	pid_t pid, wpid;
	int status;

	pid = fork();
	if (pid == 0){
		//Child process
		if (execvp(args[0], args) == -1){
			perror("clsh");
		}
		exit(EXIT_FAILURE);
	} else if (pid < 0){
		//Error forking
		perror("clsh");
	}else {
		// Parent process
		do {
			wpid = waitpid(pid, &status, WUNTRACED);
		} while (!WIFEXITED(status) && !WIFSIGNALED(status));
	}
	return 1;
}

int clsh_execute(char **args){
	int i;

	if (args[0] == NULL){
		// An empty command was entered.
		return 1;
	}

	for (i = 0; i < clsh_num_builtins(); i++) {
		if (strcmp(args[0], builtin_str[i]) == 0) {
			return (*builtin_func[i])(args);
		}
	}
	return clsh_launch(args);
}


#define CLSH_TOK_BUFSIZE 64
# define CLSH_TOK_DELIM " \t\r\n\a"
char **clsh_split_line(char *line){
	int buffsize = CLSH_TOK_BUFSIZE, position = 0;
	char **tokens = malloc(buffsize *sizeof(char*));
	char *token;

	if (!tokens){
		fprintf(stderr, "clsh: allocation error\n");
		exit(EXIT_FAILURE);
	}

	token = strtok(line, CLSH_TOK_DELIM);
	while (token != NULL){
		tokens[position] = token;
		position++;

		if (position >= buffsize){
			buffsize += CLSH_TOK_BUFSIZE;
			tokens = realloc(tokens, buffsize * sizeof(char *));
			if (!tokens){
				fprintf(stderr, "clsh: allocation error\n");
				exit(EXIT_FAILURE);
			}
		}

		token = strtok(NULL, CLSH_TOK_DELIM);
	}
	tokens[position] = NULL;
	return tokens;
}


#define CLSH_RL_BUFSIZE 1024
char *clsh_read_line(void){

	int buffsize = CLSH_RL_BUFSIZE;
	int position = 0;
	char *buffer = malloc(sizeof(char) *buffsize);
	int c;

	if (!buffer){
		fprintf(stderr, "clsh: allocation error\n");
		exit(EXIT_FAILURE);
	}

	while(1){
		// Read a character
		c = getchar();

		//IF we hit EOF, replace it with a null character and return.
		if (c == EOF || c =='\n'){
			buffer[position] = '\0';
			return buffer;
		}else {
			buffer[position] = c;
		}
		position++;

		// If we have exceeded the buffer, reallocate.

		if (position >= buffsize){
			buffsize += CLSH_RL_BUFSIZE;
			buffer = realloc(buffer, buffsize);
			if (!buffer) {
				fprintf(stderr, "clsh: allocation error\n");
				exit(EXIT_FAILURE);
			}
		}
	}
}


void clsh_loop(void){
	char *line;
	char **args;
	int status;

	do {
		printf("> ");
		line = clsh_read_line();
		args = clsh_split_line(line);
		status = clsh_execute(args);

		free (line);
		free (args);
	}while (status);

}

int main(int argc, char **argv){
	// Load config files, if any.

	//Run command loop.
	clsh_loop();

	//Perform any shutdown/cleanup


	return EXIT_SUCCESS;
}

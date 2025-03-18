#define _GNU_SOURCE

#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <pwd.h>
#include <limits.h>

#include "tokenizer.h"
#include "interpreter.h"

const char* get_username_str()
{
  struct passwd *pw = getpwuid(getuid());
  
  if (!pw) {
    fprintf(stderr, "Your user is fucked\n");
    exit(1);
  }
  
  return pw->pw_name;
}

int main(void)
{
  while (1) {
    char hostname[HOST_NAME_MAX];
    if (gethostname(hostname, HOST_NAME_MAX) < 0) {
      fprintf(stderr, "Your hostname is fucked\n");
      exit(1);
    }

    char* cwd = get_current_dir_name();
    if (!cwd) {
      fprintf(stderr, "Buy more ram\n");
      exit(1);
    }
    
    printf("[%s@%s] %s d1sh> ", get_username_str(), hostname, cwd);
    fflush(stdout);

    free(cwd);

    char* line = NULL;
    size_t line_sz = 0;
    ssize_t c_read = getline(&line, &line_sz, stdin);

    if (c_read < 0) {
      fprintf(stderr, "Buy more ream\n");
      exit(1);
    }

    line[c_read - 1] = '\0';
    interpret(tokenize(line, c_read));
  }
  
  return 0;
}

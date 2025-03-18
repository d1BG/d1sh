#define _GNU_SOURCE

#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

#include "interpreter.h"

void interpret(Token* token)
{
  Token *current = token;
  
  while (current != NULL) {
    switch (current->type) {
    case TOKEN_TYPE_EXIT:
      exit(0);
      break;
    case TOKEN_TYPE_PWD: {
      char* cwd = get_current_dir_name();
      if (!cwd) {
        fprintf(stderr, "Buy more ram\n");
        exit(1);
      }
      printf("%s\n", cwd);

      free(cwd);
    } break;
    case TOKEN_TYPE_CD:
      if (current->argc != 1) {
        fprintf(stderr, "Invalid number of arguments\n");
      }
      chdir(current->argv[0]);
      break;
    default:
      fprintf(stderr, "oopsie daisies >w<\n");
      break;
    }
    
    current = current->next;
  }

  free_tokens(token);
}

#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "tokenizer.h"

static enum token_type get_type_from_string(const char* str, size_t len)
{
  if (!strncmp(str, "exit", len)) {
    return TOKEN_TYPE_EXIT;
  } else if (!strncmp(str, "cd", len)) {
    return TOKEN_TYPE_CD;
  } else if (!strncmp(str, "pwd", len)) {
    return TOKEN_TYPE_PWD;
  } else {
    return TOKEN_TYPE_MAX;
  }
}

static void ctsi_push(char* cts, size_t* ctsi, char c)
{
  if (*ctsi == FILENAME_MAX) {
    fprintf(stderr, "Halbe Bibel, ganzer Hurensohn\n");
    exit(1);
  }

  cts[*ctsi] = c;
  (*ctsi)++;
}

enum state {
  STATE_ESCAPE = 0x1, /* 0001 */
  STATE_QUOTE  = 0x2, /* 0010 */
};

int token_push(Token** root, Token** current, char* cts, size_t* ctsi)
{
  /* We could very easily support multiple tokens
   * but the original doesn't so we don't either */
  if (*root == NULL) {
    enum token_type type = get_type_from_string(cts, *ctsi);
    if (type == TOKEN_TYPE_MAX) {
      fprintf(stderr, "Invalid command: '%s'\n", cts);
      return -1;
    }

    *root = calloc(1, sizeof(Token));
    (*root)->type = type;
    *current = *root;
  } else {
    (*current)->argc++;
          
    char** tmp = reallocarray((*root)->argv, (*root)->argc, sizeof(char*));
    if (!tmp) {
      fprintf(stderr, "Buy more ram\n");
      exit(1);
    }

    (*current)->argv = tmp;
    (*current)->argv[(*current)->argc - 1] = strndup(cts, *ctsi);
    if (!(*current)->argv[(*current)->argc - 1]) {
      fprintf(stderr, "Buy more ram\n");
      exit(1);
    }
  }

  memset(cts, 0, *ctsi);
  *ctsi = 0;

  return 0;
}

void free_tokens(Token* token)
{
  if (token == NULL) return;

  for (size_t i = 0; i < token->argc; i++)
    free(token->argv[i]);

  free(token->argv);

  free_tokens(token->next);
}

Token* tokenize(const char* input, size_t size)
{
  Token *root = NULL, *current = NULL;
  char cts[FILENAME_MAX] = "";
  size_t ctsi = 0;
  enum state state = 0;
  
  for (size_t i = 0; i < size; i++) {
    if (state & STATE_ESCAPE) {
      ctsi_push(cts, &ctsi, input[i]);
      state ^= STATE_ESCAPE;
      continue;
    }

    switch (input[i]) {
    case '\\':
      state |= STATE_ESCAPE;
      break;
    case '"':
    case '\'':
      if (state & STATE_QUOTE)
        state ^= STATE_QUOTE;
      else
        state |= STATE_QUOTE;
      break;
    case ' ': {
      if (state & STATE_QUOTE) {
        __attribute__((fallthrough));
      } else {
        if (token_push(&root, &current, cts, &ctsi) < 0)
          return NULL;
        break;
      }
    }
    default:
      ctsi_push(cts, &ctsi, input[i]);
      break;
    }
  }

  if (ctsi != 0) {
    if (token_push(&root, &current, cts, &ctsi) < 0)
      return NULL;
  }

  return root;
}

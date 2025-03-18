#pragma once

#include <stddef.h>

enum token_type {
  TOKEN_TYPE_EXIT,
  TOKEN_TYPE_PWD,
  TOKEN_TYPE_CD,
  TOKEN_TYPE_MAX,
};

struct __token {
  enum token_type type;
  size_t argc;
  char** argv;
  struct __token *next;
};
typedef struct __token Token;

Token* tokenize(const char* input, size_t size);
void free_tokens(Token* token);

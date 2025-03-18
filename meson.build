project('d1sh', 'c',
  version: '0.1.0',
  default_options: ['warning_level=3', 'c_std=gnu17'])

src = files(
  'src/main.c',
  'src/tokenizer.c',
  'src/interpreter.c',
)

inc_dirs = include_directories('src')

executable('d1sh',
  src,
  include_directories: inc_dirs,
  install: true)
# Learning projects
This repo is for project just for Learning purpose. I do not explicitly own any of the code that I've pushed here. It's to keep all code for ProjectBasedLearning found on https://github.com/tuvtran/project-based-learning.

## The further developement of this Projects

However, I do have plans to extend the projects functionality, because most of them covers only basic concepts in Computer Science, Cryptography, and other security non-related projects just for getting familiar with Programming Language.
## TREE OVERVIEW 
```.
|-- DevSecOps
|   `-- ansible-training
|       `-- Section_1_YAML
|           |-- Exercise_1
|           |   `-- yaml.yaml
|           |-- Exercise_2
|           |   `-- food.yml
|           |-- Exercise_3
|           |   `-- food.yml
|           |-- Exercise_4
|           |   `-- employee.yml
|           |-- Exercise_5
|           |   `-- employee.yml
|           `-- Exercise_6
|               `-- employee.yml
|-- Programming_Languages
|   |-- Ansible
|   |   |-- Ansible_LAB_setup.sh
|   |   `-- Vagrantfile
|   |-- Assembly
|   |-- C
|   |   |-- Bootloader
|   |   |   |-- 16-bit_Boot
|   |   |   |   |-- boot1_x86.asm
|   |   |   |   `-- boot1_x86.bin
|   |   |   `-- 32-bit_Boot
|   |   |       |-- boot2.asm
|   |   |       `-- boot2.bin
|   |   |-- Fuzzer
|   |   |   |-- fuzz.py
|   |   |   |-- program
|   |   |   `-- program.c
|   |   |-- Kernel
|   |   |   |-- kasm.o
|   |   |   |-- kc.o
|   |   |   |-- kernel-701
|   |   |   |-- kernel_entrypoint.asm
|   |   |   |-- kernel_entrypoint.o
|   |   |   |-- kmain.c
|   |   |   `-- link.ld
|   |   `-- Shell_in_c
|   |       |-- clsh_shell
|   |       `-- clsh_shell.c
|   |-- Java
|   |   `-- MazeSolver
|   |       |-- bin
|   |       |   |-- Maze.class
|   |       |   |-- MazeSolver.class
|   |       |   `-- Position.class
|   |       |-- hs_err_pid71598.log
|   |       |-- hs_err_pid72289.log
|   |       |-- mazes.txt
|   |       `-- src
|   |           |-- Maze.java
|   |           |-- MazeSolver.java
|   |           `-- Position.java
|   |-- Python
|   |   `-- Ciphers
|   |       |-- Affine_Cipher
|   |       |-- Ceasar_Cipher
|   |       |   |-- Hacking_it
|   |       |   |   `-- caesar_decryption.py
|   |       |   `-- ceasar_cipher.py
|   |       |-- Cipher_Fuzzer
|   |       |   `-- cipher_fuzzer.py
|   |       |-- Cryptomath
|   |       |   |-- __pycache__
|   |       |   |   `-- cryptomath.cpython-38.pyc
|   |       |   `-- cryptomath.py
|   |       |-- Transposition_Cipher
|   |       |   |-- Hacking_it
|   |       |   |   `-- transpositionHacker.py
|   |       |   |-- __pycache__
|   |       |   |   |-- detectEnglish.cpython-38.pyc
|   |       |   |   |-- transposition_cipher_decrypt.cpython-38.pyc
|   |       |   |   `-- transposition_cipher_encrypt.cpython-38.pyc
|   |       |   |-- cipher_fuzzer.py
|   |       |   |-- detectEnglish.py
|   |       |   |-- dictionary.txt
|   |       |   |-- frankenstein.encrypted.txt
|   |       |   |-- frankenstein.txt
|   |       |   |-- testtesttest
|   |       |   |-- transpositionHacker.py
|   |       |   |-- transposition_cipher_decrypt.py
|   |       |   |-- transposition_cipher_encrypt.py
|   |       |   `-- transposition_cipher_file.py
|   |       `-- english_detector
|   |           |-- __pycache__
|   |           |   `-- detectEnglish.cpython-38.pyc
|   |           `-- dictionary.txt
|   |-- bash_and_gnu
|   |   `-- parallel
|   |       `-- test_files
|   |           |-- abc-file
|   |           |-- abc0_file
|   |           |-- def-file
|   |           |-- example.1
|   |           |-- example.2
|   |           |-- example.3
|   |           |-- example.4
|   |           |-- example.5
|   |           |-- fixedlen
|   |           |-- myfifo
|   |           |-- num128
|   |           |-- num30000
|   |           |-- num8
|   |           |-- num_%header
|   |           `-- tsv-file.tsv
|   |-- javascript
|   |   |-- JavaScript30
|   |   |   |-- 01\ -\ JavaScript\ Drum\ Kit
|   |   |   |   |-- background.jpg
|   |   |   |   |-- index-START.html
|   |   |   |   |-- script.js
|   |   |   |   |-- sounds
|   |   |   |   |   |-- boom.wav
|   |   |   |   |   |-- clap.wav
|   |   |   |   |   |-- hihat.wav
|   |   |   |   |   |-- kick.wav
|   |   |   |   |   |-- openhat.wav
|   |   |   |   |   |-- ride.wav
|   |   |   |   |   |-- snare.wav
|   |   |   |   |   |-- tink.wav
|   |   |   |   |   `-- tom.wav
|   |   |   |   `-- style.css
|   |   |   |-- 02\ -\ JS\ and\ CSS\ Clock
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 03\ -\ CSS\ Variables
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 04\ -\ Array\ Cardio\ Day\ 1
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 05\ -\ Flex\ Panel\ Gallery
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 06\ -\ Type\ Ahead
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- style.css
|   |   |   |-- 07\ -\ Array\ Cardio\ Day\ 2
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 08\ -\ Fun\ with\ HTML5\ Canvas
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 09\ -\ Dev\ Tools\ Domination
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 10\ -\ Hold\ Shift\ and\ Check\ Checkboxes
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 11\ -\ Custom\ Video\ Player
|   |   |   |   |-- 652333414.mp4
|   |   |   |   |-- index.html
|   |   |   |   |-- scripts-FINISHED.js
|   |   |   |   |-- scripts.js
|   |   |   |   `-- style.css
|   |   |   |-- 12\ -\ Key\ Sequence\ Detection
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 13\ -\ Slide\ in\ on\ Scroll
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 14\ -\ JavaScript\ References\ VS\ Copying
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 15\ -\ LocalStorage
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   |-- oh-la-la.jpeg
|   |   |   |   `-- style.css
|   |   |   |-- 16\ -\ Mouse\ Move\ Shadow
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 17\ -\ Sort\ Without\ Articles
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 18\ -\ Adding\ Up\ Times\ with\ Reduce
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 19\ -\ Webcam\ Fun
|   |   |   |   |-- index.html
|   |   |   |   |-- package.json
|   |   |   |   |-- scripts-FINISHED.js
|   |   |   |   |-- scripts.js
|   |   |   |   |-- snap.mp3
|   |   |   |   `-- style.css
|   |   |   |-- 20\ -\ Speech\ Detection
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- package.json
|   |   |   |-- 21\ -\ Geolocation
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- package.json
|   |   |   |-- 22\ -\ Follow\ Along\ Link\ Highlighter
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- style.css
|   |   |   |-- 23\ -\ Speech\ Synthesis
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- style.css
|   |   |   |-- 24\ -\ Sticky\ Nav
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   |-- style-FINISHED.css
|   |   |   |   `-- style-START.css
|   |   |   |-- 25\ -\ Event\ Capture,\ Propagation,\ Bubbling\ and\ Once
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 26\ -\ Stripe\ Follow\ Along\ Nav
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   `-- index-START.html
|   |   |   |-- 27\ -\ Click\ and\ Drag
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- style.css
|   |   |   |-- 28\ -\ Video\ Speed\ Controller
|   |   |   |   |-- index-FINISHED.html
|   |   |   |   |-- index-START.html
|   |   |   |   `-- style.css
|   |   |   |-- 29\ -\ Countdown\ Timer
|   |   |   |   |-- index.html
|   |   |   |   |-- scripts-FINISHED.js
|   |   |   |   |-- scripts-START.js
|   |   |   |   `-- style.css
|   |   |   `-- 30\ -\ Whack\ A\ Mole
|   |   |       |-- dirt.svg
|   |   |       |-- index-FINISHED.html
|   |   |       |-- index-START.html
|   |   |       |-- mole.svg
|   |   |       `-- style.css
|   |   `-- NodeJS_Project
|   |       `-- main.js
|   `-- nim
|       |-- Chat-Application
|       |   |-- bin
|       |   |-- images
|       |   |-- src
|       |   |   |-- client
|       |   |   |-- client.nim
|       |   |   |-- client.nims
|       |   |   |-- protocol
|       |   |   |-- protocol.nim
|       |   |   |-- server
|       |   |   `-- server.nim
|       |   `-- tests
|       `-- Understanding_Parallelism
|           |-- WikipediaStats
|           |   `-- bin
|           `-- listings
|               |-- listing2
|               |-- listing2.nim
|               |-- listing3
|               |-- listing3.nim
|               |-- listing4
|               |-- listing4.nim
|               |-- listing5
|               |-- listing5.nim
|               |-- listing7_parsing_with_regex
|               |-- listing7_parsing_with_regex.nim
|               |-- listing8_parsing_with_split.nim
|               `-- listing9_parsing_with_parseutils.nim
`-- README.md

83 directories, 184 files
```
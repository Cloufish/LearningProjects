# Learning projects
This repo is for project just for Learning purpose. I do not explicitly own any of the code that I've pushed here. It's to keep all code for ProjectBasedLearning found on https://github.com/tuvtran/project-based-learning.

## The further developement of this Projects

However, I do have plans to extend the projects functionality, because most of them cover only basic concepts in Computer Science, Cryptography, and other security non-related projects just for getting familiar with Programming Language.
## TREE OVERVIEW 
```.
├── Android_Apps
│   ├── MOBISEC2020IntroDev
│   │   ├── app
│   │   │   ├── build.gradle
│   │   │   ├── proguard-rules.pro
│   │   │   └── src
│   │   │       ├── androidTest
│   │   │       │   └── java
│   │   │       │       └── com
│   │   │       │           └── example
│   │   │       │               └── mobisec2020introdev
│   │   │       │                   └── ExampleInstrumentedTest.java
│   │   │       ├── main
│   │   │       │   ├── AndroidManifest.xml
│   │   │       │   ├── java
│   │   │       │   │   └── com
│   │   │       │   │       └── example
│   │   │       │   │           └── mobisec2020introdev
│   │   │       │   │               ├── BumoActivity.java
│   │   │       │   │               ├── First2Fragment.java
│   │   │       │   │               ├── FirstFragment.java
│   │   │       │   │               ├── MainActivity.java
│   │   │       │   │               ├── Second2Fragment.java
│   │   │       │   │               └── SecondFragment.java
│   │   │       │   └── res
│   │   │       │       ├── drawable
│   │   │       │       │   └── ic_launcher_background.xml
│   │   │       │       ├── drawable-v24
│   │   │       │       │   └── ic_launcher_foreground.xml
│   │   │       │       ├── layout
│   │   │       │       │   ├── activity_bumo.xml
│   │   │       │       │   ├── activity_main.xml
│   │   │       │       │   ├── content_bumo.xml
│   │   │       │       │   ├── content_main.xml
│   │   │       │       │   ├── fragment_first.xml
│   │   │       │       │   ├── fragment_first2.xml
│   │   │       │       │   ├── fragment_second.xml
│   │   │       │       │   └── fragment_second2.xml
│   │   │       │       ├── menu
│   │   │       │       │   └── menu_main.xml
│   │   │       │       ├── mipmap-anydpi-v26
│   │   │       │       │   ├── ic_launcher.xml
│   │   │       │       │   └── ic_launcher_round.xml
│   │   │       │       ├── mipmap-hdpi
│   │   │       │       │   ├── ic_launcher.png
│   │   │       │       │   └── ic_launcher_round.png
│   │   │       │       ├── mipmap-mdpi
│   │   │       │       │   ├── ic_launcher.png
│   │   │       │       │   └── ic_launcher_round.png
│   │   │       │       ├── mipmap-xhdpi
│   │   │       │       │   ├── ic_launcher.png
│   │   │       │       │   └── ic_launcher_round.png
│   │   │       │       ├── mipmap-xxhdpi
│   │   │       │       │   ├── ic_launcher.png
│   │   │       │       │   └── ic_launcher_round.png
│   │   │       │       ├── mipmap-xxxhdpi
│   │   │       │       │   ├── ic_launcher.png
│   │   │       │       │   └── ic_launcher_round.png
│   │   │       │       ├── navigation
│   │   │       │       │   ├── nav_graph.xml
│   │   │       │       │   └── nav_graph2.xml
│   │   │       │       ├── values
│   │   │       │       │   ├── colors.xml
│   │   │       │       │   ├── dimens.xml
│   │   │       │       │   ├── strings.xml
│   │   │       │       │   └── themes.xml
│   │   │       │       └── values-night
│   │   │       │           └── themes.xml
│   │   │       └── test
│   │   │           └── java
│   │   │               └── com
│   │   │                   └── example
│   │   │                       └── mobisec2020introdev
│   │   │                           └── ExampleUnitTest.java
│   │   ├── build.gradle
│   │   ├── gradle
│   │   │   └── wrapper
│   │   │       ├── gradle-wrapper.jar
│   │   │       └── gradle-wrapper.properties
│   │   ├── gradle.properties
│   │   ├── gradlew
│   │   ├── gradlew.bat
│   │   └── settings.gradle
│   └── ToDoList
│       ├── app
│       │   ├── build.gradle
│       │   ├── proguard-rules.pro
│       │   └── src
│       │       ├── androidTest
│       │       │   └── java
│       │       │       └── com
│       │       │           └── example
│       │       │               └── todoactivity
│       │       │                   └── ExampleInstrumentedTest.java
│       │       ├── main
│       │       │   ├── AndroidManifest.xml
│       │       │   ├── java
│       │       │   │   └── com
│       │       │   │       └── example
│       │       │   │           └── todoactivity
│       │       │   │               ├── Adapter
│       │       │   │               │   └── ToDoAdapter.java
│       │       │   │               ├── AddNewTask.java
│       │       │   │               ├── DialogCloseListener.java
│       │       │   │               ├── MainActivity.java
│       │       │   │               ├── Model
│       │       │   │               │   └── ToDoModel.java
│       │       │   │               ├── RecyclerItemTouchHelper.java
│       │       │   │               ├── SplashActivity.java
│       │       │   │               └── Utils
│       │       │   │                   └── DatabaseHandler.java
│       │       │   └── res
│       │       │       ├── drawable
│       │       │       │   ├── ic_baseline_add.xml
│       │       │       │   ├── ic_baseline_delete_24.xml
│       │       │       │   ├── ic_baseline_edit.xml
│       │       │       │   └── ic_launcher_background.xml
│       │       │       ├── drawable-v24
│       │       │       │   └── ic_launcher_foreground.xml
│       │       │       ├── layout
│       │       │       │   ├── activity_main.xml
│       │       │       │   ├── activity_splash.xml
│       │       │       │   ├── new_task.xml
│       │       │       │   └── task_layout.xml
│       │       │       ├── layout-v21
│       │       │       │   ├── activity_main.xml
│       │       │       │   └── task_layout.xml
│       │       │       ├── mipmap-anydpi-v26
│       │       │       │   ├── ic_launcher.xml
│       │       │       │   └── ic_launcher_round.xml
│       │       │       ├── mipmap-hdpi
│       │       │       │   ├── ic_launcher.png
│       │       │       │   └── ic_launcher_round.png
│       │       │       ├── mipmap-mdpi
│       │       │       │   ├── ic_launcher.png
│       │       │       │   └── ic_launcher_round.png
│       │       │       ├── mipmap-xhdpi
│       │       │       │   ├── ic_launcher.png
│       │       │       │   └── ic_launcher_round.png
│       │       │       ├── mipmap-xxhdpi
│       │       │       │   ├── ic_launcher.png
│       │       │       │   └── ic_launcher_round.png
│       │       │       ├── mipmap-xxxhdpi
│       │       │       │   ├── ic_launcher.png
│       │       │       │   └── ic_launcher_round.png
│       │       │       ├── values
│       │       │       │   ├── colors.xml
│       │       │       │   ├── strings.xml
│       │       │       │   └── themes.xml
│       │       │       └── values-night
│       │       │           └── themes.xml
│       │       └── test
│       │           └── java
│       │               └── com
│       │                   └── example
│       │                       └── todoactivity
│       │                           └── ExampleUnitTest.java
│       ├── build.gradle
│       ├── gradle
│       │   └── wrapper
│       │       ├── gradle-wrapper.jar
│       │       └── gradle-wrapper.properties
│       ├── gradle.properties
│       ├── gradlew
│       ├── gradlew.bat
│       └── settings.gradle
├── DevSecOps
│   ├── Docker_Stuff
│   │   └── Apache_HTTPd_Server
│   │       ├── Dockerfile
│   │       └── httpd.conf
│   └── Kubernetes_Stuff
│       └── Intro
│           └── deployment.yml
├── Programming_Languages
│   ├── Android_Apps
│   │   ├── build.gradle
│   │   ├── gradle.properties
│   │   ├── gradlew
│   │   ├── gradlew.bat
│   │   └── settings.gradle
│   ├── Ansible
│   │   ├── Ansible_LAB_setup.sh
│   │   └── Vagrantfile
│   ├── C
│   │   ├── Bootloader
│   │   │   ├── 16-bit_Boot
│   │   │   │   ├── boot1_x86.asm
│   │   │   │   └── boot1_x86.bin
│   │   │   └── 32-bit_Boot
│   │   │       ├── boot2.asm
│   │   │       └── boot2.bin
│   │   ├── Fuzzer
│   │   │   ├── fuzz.py
│   │   │   ├── program
│   │   │   └── program.c
│   │   ├── Kernel
│   │   │   ├── kasm.o
│   │   │   ├── kc.o
│   │   │   ├── kernel-701
│   │   │   ├── kernel_entrypoint.asm
│   │   │   ├── kernel_entrypoint.o
│   │   │   ├── kmain.c
│   │   │   └── link.ld
│   │   └── Shell_in_c
│   │       ├── clsh_shell
│   │       └── clsh_shell.c
│   ├── Java
│   │   └── MazeSolver
│   │       ├── mazes.txt
│   │       └── src
│   │           ├── Maze.java
│   │           ├── MazeSolver.java
│   │           └── Position.java
│   ├── Python
│   │   └── Ciphers
│   │       ├── Ceasar_Cipher
│   │       │   ├── Hacking_it
│   │       │   │   └── caesar_decryption.py
│   │       │   └── ceasar_cipher.py
│   │       ├── Cipher_Fuzzer
│   │       │   └── cipher_fuzzer.py
│   │       ├── Cryptomath
│   │       │   ├── __pycache__
│   │       │   │   └── cryptomath.cpython-38.pyc
│   │       │   └── cryptomath.py
│   │       ├── Transposition_Cipher
│   │       │   ├── Hacking_it
│   │       │   │   └── transpositionHacker.py
│   │       │   ├── __pycache__
│   │       │   │   ├── detectEnglish.cpython-38.pyc
│   │       │   │   ├── transposition_cipher_decrypt.cpython-38.pyc
│   │       │   │   └── transposition_cipher_encrypt.cpython-38.pyc
│   │       │   ├── cipher_fuzzer.py
│   │       │   ├── detectEnglish.py
│   │       │   ├── dictionary.txt
│   │       │   ├── frankenstein.encrypted.txt
│   │       │   ├── frankenstein.txt
│   │       │   ├── testtesttest
│   │       │   ├── transpositionHacker.py
│   │       │   ├── transposition_cipher_decrypt.py
│   │       │   ├── transposition_cipher_encrypt.py
│   │       │   └── transposition_cipher_file.py
│   │       └── english_detector
│   │           ├── __pycache__
│   │           │   └── detectEnglish.cpython-38.pyc
│   │           └── dictionary.txt
│   ├── bash_and_gnu
│   │   └── parallel
│   │       └── test_files
│   │           ├── abc-file
│   │           ├── abc0_file
│   │           ├── def-file
│   │           ├── example.1
│   │           ├── example.2
│   │           ├── example.3
│   │           ├── example.4
│   │           ├── example.5
│   │           ├── fixedlen
│   │           ├── num128
│   │           ├── num30000
│   │           ├── num8
│   │           ├── num_%header
│   │           └── tsv-file.tsv
│   ├── javascript
│   │   └── NodeJS_Project
│   │       └── main.js
│   └── nim
│       ├── Chat-Application
│       │   └── src
│       │       ├── client
│       │       ├── client.nim
│       │       ├── client.nims
│       │       ├── protocol
│       │       ├── protocol.nim
│       │       ├── server
│       │       └── server.nim
│       └── Understanding_Parallelism
│           └── listings
│               ├── listing2
│               ├── listing2.nim
│               ├── listing3
│               ├── listing3.nim
│               ├── listing4
│               ├── listing4.nim
│               ├── listing5
│               ├── listing5.nim
│               ├── listing7_parsing_with_regex
│               ├── listing7_parsing_with_regex.nim
│               ├── listing8_parsing_with_split.nim
│               └── listing9_parsing_with_parseutils.nim
├── README.md
└── Web_Apps
    └── MEAN-Stack
        ├── README.md
        ├── angular.json
        ├── e2e
        │   ├── protractor.conf.js
        │   ├── src
        │   │   ├── app.e2e-spec.ts
        │   │   └── app.po.ts
        │   └── tsconfig.json
        ├── karma.conf.js
        ├── package-lock.json
        ├── package.json
        ├── src
        │   ├── app
        │   │   ├── app-routing.module.ts
        │   │   ├── app.component.spec.ts
        │   │   ├── app.component.ts
        │   │   ├── app.module.ts
        │   │   └── posts
        │   │       ├── header
        │   │       │   └── header.component.ts
        │   │       ├── post-create
        │   │       │   └── post-create.component.ts
        │   │       └── posts-list
        │   │           └── posts-list.component.ts
        │   ├── assets
        │   ├── environments
        │   │   ├── environment.prod.ts
        │   │   └── environment.ts
        │   ├── favicon.ico
        │   ├── main.ts
        │   ├── polyfills.ts
        │   └── test.ts
        ├── tsconfig.app.json
        ├── tsconfig.json
        ├── tsconfig.spec.json
        └── tslint.json

123 directories, 207 files
```

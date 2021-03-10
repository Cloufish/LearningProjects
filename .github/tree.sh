#!/bin/bash

echo  "# Learning projects" > README.md
echo -n "This repo is for project just for Learning purpose. I do not explicitly own any of the code that I've pushed here. It's to keep all code for ProjectBasedLearning found on https://github.com/tuvtran/project-based-learning.

## The further developement of this Projects

However, I do have plans to extend the projects functionality, because most of them covers only basic concepts in Computer Science, Cryptography, and other security non-related projects just for getting familiar with Programming Language.
" >> README.md
echo  "## TREE OVERVIEW " >> README.md
echo -n "\`\`\`" >> README.md
tree >> README.md
echo -n "\`\`\`" >> README.md
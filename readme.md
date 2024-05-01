---
title: Vim Motion
author: Sanath 
date: 2019-11-19
---

# What is vim-motion?

- It's not vim shortcuts, it's a way of thinking 
- It's a language to navigate and manipulate text

# Important Vim Motion Concepts
         ↑
         k         
    ← h     l →          
         j              
         ↓
- These prefix with a count to move multiple times


# Important Vim Motion Concepts
- Some termanology
    - `y` = yank (copy)
    - `p` = paste
    - `d` = delete
    - `c` = change
    - `t` = to
    - `f` = find
    - `a` = around
    - `w` = word
    - etc



# Important Vim Motion Concepts
- Motion can be combined with operators
    - `d` + `w` = delete word
    - `c` + `i` + `(` = change inside parentheses

- Also motion is composable as 
    - {`operator`}{`count`}{`motion`} = `d2w` = delete 2 words
    - {`count`}{`operator`}{`motion`} = `2dw` = delete 2 words

# Text Objects
- Text objects are a way to select text based on its structure
- Text objects motion can be used with operators, `a` = around, `i` = inside
    - {`operator`}{`a`|`i`}{`text-object`} = `ciw` = change inside word
    - `daw` = delete a word
    - `ciw` = change inside word


# Why Vim Motion?

- Vim motion is not limited to vim editor, you can use it in any editor VsCode, IntelliJ etc
- More efficient than using mouse
- Once you get used to it, you can't go back
- It's fun to use
- So many tools support vim motion
    - obsidian



# Resources
- Presentation: https://github.com/sanathks/vim-motion 
- VsCode extention : https://github.com/VSCodeVim/Vim
- JetBrains plugin : https://github.com/JetBrains/ideavim
- Vimtutor : https://www.openvim.com/tutorial.html

![image](https://github.com/sanathks/vim-motion/assets/4918600/f36351a6-069c-44a3-bcd7-3f36085f6c00)

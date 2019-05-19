# LLLE - Ludicrously Large Line Editor

# NOT EVEN CLOSE TO FUNCTIONAL (... yet?) But should you actually care?

This is a silly side-project based off of [rote](https://github.com/Ryan1729/rote) @ [6b0ac47](https://github.com/Ryan1729/rote/commit/6b0ac471eba8ecf4ff4e16aa3d5b7bfcc405cd3c).

The goal is to make a graphical text editor which shows only a single line at a time but otherwise has all the basic features: text selection, cut/copy/paste, and opening and saving files. Maybe undo/redo?

## Plan
* Implement everything in [MVP-FEATURES.md](./design/MVP-FEATURES.md).
* Go back to working on rote?

## Motiviation
While working on rote I changed out the text buffer backend for performance reasons, and saw essentially the same line ending bugs as I had before. I found this frustrating/demoralizing. I had recently read [this blog post](https://www.drmaciver.com/2019/05/how-to-do-hard-things/) which describes "The Fully General System For Learning To Do Hard Things". The forst step there is "Find something that is like the hard thing but is easy." This made me think about how much easier it would be to write an editor that only had to deal with a single line. I was reminded of the line editor like [ed](https://en.wikipedia.org/wiki/Ed_(text_editor)) and their bizarre (to modern eyes?) interface. This combination of trying to simplify the project and strange UIs lead me to the idea of an editor that works like a modern one, but only shows a single line. Since the focus of the editor's display should be on the text, "obviously" the text must take up most of the screen. The idea of an editor that takes up the whole scren but only has one line was so wonderfully absurd that I wanted to make it a reality.

____

Licensed under MIT and Apache 2.0
The OpenGL platform layer is licensed under Apache 2.0

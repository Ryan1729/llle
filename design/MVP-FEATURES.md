# Minimum features to feel like a real (if real-weird) text editor.

## Standard features
* Shift and arrow keys to select text
* Home and End to jump cursors to start and end of line
* Ctrl, and left and right arrow keys to jump cursor past words
* Ctrl-x to cut
* Ctrl-c to copy
* Ctrl-v to paste
* Mouse selection of text
* open/save

## Unique features
If we want to make this halfway usable then we might need specialized features for this editing paradigm.
* Ctrl-g Go to word number or otherwise jump around the file

# Design considerations
Oddly enough, peole sometimes need to edit files that have more than one line. These people also cannot seem to agree on what line ending to use. So let's display the line ending characters and allow editing as if they were a single line.
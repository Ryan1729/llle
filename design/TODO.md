## TODO

* fix bugs in "replace text inside regions when typing" impl
    * shift seems to be sticking (potentially unrelated?)
        * specifically pressing the arrow keys without shift pressed seems to be acting as if shift is pressed (extending the selection instead of removing it.)
        * this also seems to be affecting the Home and End keys.
    * typing when the highlight position is before the cursor makes the cursor disappear if there is nothing to the right of the selection, and it places the cursor and the resulting character in the wrong position if there is not.

* Shift and arrow keys to select text
    * replace text inside regions when typing

* fix cursor being visible through status line (!?)

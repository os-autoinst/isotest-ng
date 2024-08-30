# isototest Documentation

`isototest` is a library which handles the execution of test modules on openQA worker machines.

It's responsibilities include:

- Execution of individual test cases
- Passing input (mostly keystrokes and clicks) to the test machine via VNC
- Returning the output of the test machine to the caller
- Returning the test result to the caller

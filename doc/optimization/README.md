# Optimization

Different designs of inner interpreter.

* new-run: function call with dfa, stack and stack pointer as arguments.
* new-ip: like new-run but optimized around run() and execute_word(), ie. around ip, in order to preserve the current multitasker.
* bytecode: switch threaded code.
* tco: use same stack frame as caller through tail/sibling call optimization. Still not available with rust compiler.
* call: subroutine threaded call
* asm: use asm! to write a small subroutine threaded forth core..

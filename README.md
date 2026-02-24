# turbocollatz
An efficient CLI tool for single-threaded verification of the Collatz conjecture.

I wrote `turbocollatz` to demonstrate how powerful modern CPUs are, but apparently even I underestimated how fast they can be.

Here are some benchmark results on my Ubuntu 25.10 machine with an AMD Ryzen 9 3950X processor (your mileage may vary):
| Command    | Total time |
| -------- | ------- |
| `time ./turbocollatz 50000`  | 0.014 seconds   |
| `time ./turbocollatz 1 50000` | 0.026 seconds     |
| `time USE_SLOW=true ./turbocollatz 1 50000`    | 0.538 seconds    |

For more information, read the manpage at https://mojavesoft.net/man/1/turbocollatz.1.html or run `curl https://mojavesoft.net/man/1/turbocollatz.1 | man -l /dev/stdin` in the terminal.

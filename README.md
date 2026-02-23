# turbocollatz
An efficient CLI tool for single-threaded verification of the Collatz conjecture.

I wrote `turbocollatz` to demonstrate how powerful modern CPUs are, but apparently even I underestimated how fast they can be.

Here are some benchmark results on my Ubuntu 25.10 machine with an AMD Ryzen 9 3950X processor (your mileage may vary):
| Command    | Total time |
| -------- | ------- |
| `time ./turbocollatz 50000`  | 0.014 seconds   |
| `time ./turbocollatz 1 50000` | 0.026 seconds     |
| `time USE_SLOW=true ./turbocollatz 1 50000`    | 0.538 seconds    |

## Usage
`./turbocollatz START [END]` (if _END_ is not specified, the steps to verify the conjecture starting at _START_ until convergence to 1 will be printed to `stdout`).

### Disbling caching functionality
By default, `turbocollatz` will dynamically allocate memory when verifying a range in order to speed up calculations. While this can significantly improve performance for smaller ranges, sometimes the cache will get too large and your system will run out of memory.

If this happens to you, run `turbocollatz` with the `USE_SLOW` environment variable set to `true`. This disables caching and usually keeps memory usage below ~1 megabyte at the cost of speed.

# turbocollatz
An efficient CLI tool for single-threaded verification of the Collatz conjecture.

I wrote `turbocollatz` to demonstrate how powerful modern CPUs are, but apparently even I underestimated how fast they can be.

Here are some benchmark results on my Ubuntu 25.10 machine with an AMD Ryzen 9 3950X processor (your mileage may vary):
| Command    | Total time |
| -------- | ------- |
| `time ./turbocollatz 50000`  | 0.014 seconds   |
| `time ./turbocollatz 1 50000` | 0.026 seconds     |
| `time USE_SLOW=true ./turbocollatz 1 50000`    | 0.538 seconds    |

More documentation coming soon.

## A note about manmade code
The owner of this project believes in good faith that it complies with [The Manmade Software Declaration 1.0](https://mojavesoft.net/ai-policy/1.0).
Contributors are encouraged to follow the guidelines described at the aforementioned link when proposing any code changes, and patches that appear to violate those rules may be rejected at any time.

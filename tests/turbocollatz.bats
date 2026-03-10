#!/usr/bin/env bats
regex="Finished in .* steps\."
bats_require_minimum_version 1.5.0
@test "no arguments" {
  run -64 ../target/x86_64-unknown-linux-musl/release/turbocollatz
}

@test "one argument" {
  result="$(../target/x86_64-unknown-linux-musl/release/turbocollatz 5|head -n 1)"
  [[ "$result" == "16" ]]
}

@test "two arguments" {
  result="$(../target/x86_64-unknown-linux-musl/release/turbocollatz 1 500)"
  [[ "$result" =~ $regex ]]
}

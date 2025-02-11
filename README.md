```shell
# Executing will cause segv on ARM, but will be ok on x86
cargo b && cd target/debug && ./testapp
Segmentation fault (core dumped)
```

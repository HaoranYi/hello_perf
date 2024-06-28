Example program to demonstrate 'perf' a rust program.

1. build with 'force-frame-pointer=yes' and add 'debug=true' for release build
   in Cargo.toml

```
  RUSTFLAGS="-C force-frame-pointers=yes" cargo build --release
```

2. run with 'perf record'

```
 sudo perf record -o perf_release.data -g ./target/release/hello_perf
```

3. analyze with 'perf report'

```
  sudo perf report -i perf_release.data
```

4. view it in speedscope (locally)


```
  sudo perf script -i perf_release.data | speedscope -

```

Or dump into file and view it in the browser (https://www.speedscope.app/)

```
  sudo perf script -i perf_release.data > perf.out 
```

[speedscope] https://github.com/jlfwong/speedscope#usage

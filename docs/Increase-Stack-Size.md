## Conversation table

| MB  | Bytes       | Bits          | hexadecimal |
| --- | ----------- | ------------- | ----------- |
| 1   | 1 048 576   | 8 388 608     | 0x800000    |
| 2   | 2 097 152   | 16 777 216    | 0x1000000   |
| 4   | 4 194 304   | 33 554 432    | 0x2000000   |
| 8   | 8 388 608   | 67 108 864    | 0x4000000   |
| 16  | 16 777 216  | 134 217 728   | 0x8000000   |
| 32  | 33 554 432  | 268 435 456   | 0x10000000  |
| 64  | 67 092 480  | 536 870 912   | 0x40000000  |
| 128 | 134 217 728 | 1 073 741 824 | 0x80000000  |

## Setting the size

See `build.rs` and `.cargo/config.toml`

Setting `ulimit -s unlimited` is important. On macOS,
this will mean a stack size of 64 MB.

## Inspection

```
$ ulimit -s unlimited
$ ulimit -s
65520
```

This means 65520 \* 1024 bytes = 67 092 480 bytes = ~64 MB stack size is now allowed.

```
$ otool -lV ./target/debug/run  | grep stack
stacksize 536870912
```

This means 536870912 bits = 64 MB stack size is now allowed for the binary.

## References

- https://doc.rust-lang.org/unstable-book/compiler-flags/emit-stack-sizes.html

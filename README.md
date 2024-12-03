# Touch for Windows

This is a Rust implementation for Windows of the `touch` command found on UNIX-like operating systems.

> [!IMPORTANT]
> This is code written by a beginner in Rust for learning purposes.
> I'm sure the experienced Rustaceans here know better ways to write it.
> If that's the case, I would be grateful if you could kindly offer advice through the Issues section.

## Usage

```
$ touch <file>
```

If the file specified in the argument does not exist, it will be created.
If the file already exists, its timestamp will be updated.

### -c

If you specify the option `c`, the file will not be created if it does noe exist.
If it already exists, the timestamp will be updated as usual.

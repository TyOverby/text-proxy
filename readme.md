# Text Proxy

A program that is similar to `tee` that acts as a logging proxy for the stdio for a program.

## Usage

```
text-proxy <command> [<arg>...]
```

Runs `<command>` with the arguments provided.  Stdio and stdout are copied to in.txt and out.txt respectively.

## Example

```
# Text Proxy

A program that is similar to `tee` that acts as a logging proxy for the stdio for a program.

## Usage

```
text-proxy <command> [<arg>...]
```

Runs `<command>` with the arguments provided.  Stdio and stdout are copied to in.txt and out.txt respectively.

## Example


```
text-proxy.exe cmd /C C:\Users\Ty\AppData\Roaming\npm\tsserver.cmd
```

This spawns the typescript language server in a way that all inputs and outputs are logged.  This can be plugged
into a text editor and see what commands its sending!

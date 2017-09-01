use std::env::args;
use std::thread::spawn;
use std::fs::File;
use std::io::{copy, stdin, stdout, Read, Result as IoResult, Write};
use std::process::{exit, Command, Stdio};

struct TeeReader<R, W>(R, W);
struct TeeWriter<W1, W2>(W1, W2);

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let size = self.0.read(buf)?;
        self.1.write(&buf[..size])?;
        Ok(size)
    }
}

impl<W1: Write, W2: Write> Write for TeeWriter<W1, W2> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let len = self.0.write(buf)?;
        self.1.write_all(&buf[..len])?;
        Ok(len)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.0.flush()?;
        self.1.flush()?;
        Ok(())
    }
}


fn main() {
    let mut args = args();
    args.next(); // clear own name
    let first = args.next().unwrap();

    let mut command = Command::new(first);
    for arg in args {
        command.arg(arg);
    }

    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();
    let my_stdin = stdin();
    let my_stdout = stdout();

    spawn(move || {
        let my_stdin = my_stdin.lock();
        let file = File::create("in.txt").unwrap();
        copy(&mut TeeReader(my_stdin, file), &mut child_stdin).unwrap();
    });

    spawn(move || {
        let my_stdout = my_stdout.lock();
        let out = File::create("out.txt").unwrap();
        copy(&mut child_stdout, &mut TeeWriter(my_stdout, out)).unwrap();
    });

    let exit_code = child
        .wait()
        .ok()
        .and_then(|status| status.code())
        .unwrap_or(-1);
    exit(exit_code);
}

use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn netstat_ports() {
    let mut cmd_netstat = Command::new("netstat")
        .arg("-Watnlv")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_grep = Command::new("grep")
        .arg("LISTEN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = cmd_netstat.stdout {
        if let Some(ref mut stdin) = cmd_grep.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }

    let res = cmd_grep.wait_with_output().unwrap().stdout;
    let res_str = String::from_utf8(res).unwrap();
    let lines = res_str.lines();

    for line in lines {
        let cols = line.split_whitespace();
        println!("\n");
        println!("Line!");
        for col in cols {
            println!("Col: {:?}", col);
        }
        println!("\n");
    }
}

fn main() {
    netstat_ports();
}

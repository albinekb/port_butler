use anyhow;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

mod locations;
mod ports;

fn netstat_ports(
    range: (ports::PortNumber, ports::PortNumber),
) -> Result<ports::PortList, anyhow::Error> {
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

    let mut port_list: ports::PortList = vec![];

    for line in lines {
        let cols = line.split_whitespace();

        let mut i = 0;
        for col in cols {
            if i == 3 {
                let part = col.rsplit(".").next().unwrap_or("");
                let port = ports::Port::from_str(part).unwrap();
                let (lower, upper) = range;

                if port.number < upper && port.number > lower {
                    port_list.push(ports::probe_local_port(port).unwrap());
                }
            }
            i = i + 1;
        }
    }

    Ok(port_list)
}

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let port_list: ports::PortList = netstat_ports((2999, 4999)).unwrap();
    for port in port_list {
        let loc = locations::head_port_local(port).await;
        println!("{:?}", loc);
    }
    Ok(())
}

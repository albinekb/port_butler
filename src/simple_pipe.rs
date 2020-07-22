// use std::io::{Read, Write};
// use std::process::{Child, Command, Stdio};
// use std::str::Lines;

// pub fn pipe_cmd<'a>(list: &mut Vec<String>, mut cmd_out: Child, mut cmd_in: Child) -> String {
//   if let Some(ref mut stdout) = cmd_out.stdout {
//     if let Some(ref mut stdin) = cmd_in.stdin {
//       let mut buf: Vec<u8> = Vec::new();
//       stdout.read_to_end(&mut buf).unwrap();
//       stdin.write_all(&buf).unwrap();
//     }
//   }

//   let res = cmd_in.wait_with_output().unwrap().stdout;

//   return res;
// }

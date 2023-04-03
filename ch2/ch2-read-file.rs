use std::fs::File;
// BufReader负责提供I/O（buffered I/O），这样可以减少对操作系统的调用，也就是说减少对硬盘的读取次数。
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines {
        let line = line_.unwrape();
        println!("{} ({} bytes long)", line, len);
    }

    // let mut line = String::new();
    //
    // loop {
    //     let len = reader.read_line(&mut line).unwrap();
    //     if len == 0 {
    //         break;
    //     }
    //     println!("{} ({} bytes long)", line, len);
    //     line.truncate(0); // 将String收缩到长度为0，防止有之前行的内容遗留下来。
    // }
}
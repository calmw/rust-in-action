use regex::Regex;    // 从regex中导入Regex类型到当前局部作用域

fn main() {
    let re = Regex::new("picture").unwrap(); // 用unwrap解包装一个Result，如果有错误发生则程序崩溃。

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {    // <3>

            Some(_) => println!("{}", line),    // 代表re.find()方法查找成功。Some中的_是通配符，匹配所有值
            None => (),    // 空的占位符的值
        }
    }
}

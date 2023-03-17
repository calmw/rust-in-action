fn main() {
    let mut letters = vec!["a", "b", "c"];// 创建一个可变的动态数组

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());  // <2>
    }
}

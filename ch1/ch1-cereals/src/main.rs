#[derive(Debug)]  // 允许使用println!宏来输出枚举体Cereal
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // 初始化一个空的动态数组，其元素类型为Cereal
    grains.push(Cereal::Rye);     // 向动态数组grains（粮食）中添加元素
    drop(grains);          // 删除grains和其中的数据

    println!("{:?}", grains);
}

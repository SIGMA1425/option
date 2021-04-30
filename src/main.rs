fn main() {
    //Some(T)の値を取り出すときはunwrapが楽
    //Noneに対してunwrapを使うとpanic
    let x = double(100).unwrap();
    println!("x is{}", x);

    let x = double(20);

    //列挙型なのでパターンマッチ可
    match x{
        Some(i) => println!("x is {}", i),
        None => println!("failed"),
    }

    //if letを用いたパターンマッチ
    if let Some(i) = double(10){
        println!("x is {}", i);
    }
}

//値が格納されているかどうかを示すOption型
fn double(x:u32) -> Option<u32>{
    if x > (u32::MAX / 2){
        return None;
        //値が格納されていないときはNone
    }
    Some(x * 2)
    //値が格納されているときはSome(T)
}



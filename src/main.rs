#[allow(dead_code)]
#[allow(unused_variables)] // x bir değeri kullanmadığında unused variables hatası veriyor.

//import işlemleri
use std::mem;

fn main() {
    let a:u8 = 125; // u = unsigned yani işaretsiz demek yani 0-255 araında değer alacak 8'de 8 bit olduğunu ifade ediyoru
    println!("a = {}", a);
    // u => unsigned, 0-2^N-1'e kadar

    let b:i8 = 0; // signed değerler i ile başlıyor
    // -128=>+127'e kadar değerler alıyor.
    // -2^(N-1)'den 2^(N-1)-1'e kadar i yani işaretliler değerleri alıyor
    println!("b = {}", b);

    // eğer bir değişkenin immutable formundan kurtarıp sonrasında
    // mutable yapmak istiyorsak let mut b:i8 olarak tanımlıyoruz.

    let mut c:i32 = 10;
    println!("first c = {} and memory usage = {} byte", c, mem::size_of_val(&c));
    c = 5;
    println!("first c = {} and memory usage = {} byte", c, mem::size_of_val(&c));

}
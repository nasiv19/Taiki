use std::io; //ใช้ library เพื่อทำการ Input/output

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    // สร้างตัวแปร ใช้คำสั่ง let จะเป็น imutable
    // Ex:  let myvariable = 12;
    // แต่ถ้าต้องการเปลี่ยนแปลงค่า ต้องใช่ let mut ตามตัวอย่าง
    // Ex:  let mut myvariable = 12;
    // String::new() สร้าง instance ของ String ขึ้นมาเพื่อรับค่าจาก การ Input ของ user
    io::stdin()
        .read_line(&mut guess)
        //read_line() รับข้อมูลทุกอย่างที่ผู้ใช้พิมพ์เข้ามาทางอินพุตมาตรฐาน แล้วนำไป ต่อท้าย ลงในสตริงนั้น โดย ไม่เขียนทับเนื้อหาเดิม
        // &mut guess ตัว & เป็น reference ไปยังข้อมูล
        // &mut reference แล้ว แก้ไขข้อมูลใน guess
        // read_line() จะ return ค่า Result ซุึ้งมีีผลเป็น OK กับ Err
        .expect("Failed to read line {guess}",);
        // Result->Err จะเรียก expect จะทำให้โปรแกรมหยุดการทำงานทันที และแสดงข้อความ
        // Result->OK  จะคืนค่าออกมา
    println!("You guessed: {}", guess);
}

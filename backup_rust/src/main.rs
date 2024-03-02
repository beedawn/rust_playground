
use std::io;
use std::fs;
use std::io::Write;

const file_name:&str="backup.txt";
fn main() {
    let mut input_string = String::new();
    let mut confirm = String::new();
    println!("Hello, world!");
    io::stdin().read_line(&mut input_string).unwrap();

    println!("You wrote {}", input_string);
    println!("Overwrite(o) or append(a)?");
    io::stdin().read_line(&mut confirm).unwrap();
    confirm = confirm.to_lowercase();
    match &confirm[..] {
         "append\n"|"a\n" => {let x = file_append(input_string); println!("{:?}",x);()},
         "overwrite\n"|"o\n" =>{let x = file_write(input_string);println!("{:?}",x); ()},
        _ => println!("Nothing!"),
    }

    //file_write(input_string);
}


fn file_write(input_string:String)->std::io::Result<()>{
       let mut file = fs::File::create("backup.txt")?; 

    file.write_all(input_string.as_bytes()).expect("error writing file");
    Ok(())
}



fn file_append(input_string:String)->std::io::Result<()>{
    let mut file = fs::File::options().append(true).open(file_name)?;
    writeln!(&mut file,"{}",input_string)?;
    Ok(())

}

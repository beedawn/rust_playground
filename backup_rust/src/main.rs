
use std::io;
use std::fs;
use std::io::Write;

//const file_name:&str="backup.txt";
fn main() {
    let mut input_string = String::new();
    let mut confirm = String::new();
    let mut file_name = String::new();
    println!("Enter filename:");
    io::stdin().read_line(&mut file_name).unwrap();
    file_name=file_name.as_str().trim().to_string();
    println!("Hello, world!");
    io::stdin().read_line(&mut input_string).unwrap();
    input_string=input_string.as_str().trim().to_string();
    println!("You wrote {}", input_string);
    println!("Overwrite(o) or append(a)?");
    io::stdin().read_line(&mut confirm).unwrap();
    confirm = confirm.to_lowercase();
    match &confirm[..] {
         "append\n"|"a\n" => {let x = file_append(input_string, file_name); println!("{:?}",x);},
         "overwrite\n"|"o\n" =>{let x = file_write(input_string, file_name);println!("{:?}",x); },
        _ => println!("Nothing!"),
    }

    //file_write(input_string);
}


fn file_write(input_string:String, file_name:String)->std::io::Result<()>{
       let mut file = fs::File::create(file_name)?; 
    writeln!(&mut file,"{}",input_string)?;
    //file.write_all(input_string.as_bytes()).expect("error writing file");
    Ok(())
}



fn file_append(input_string:String, file_name:String)->std::io::Result<()>{
    let mut file = fs::File::options().append(true).open(file_name)?;
    writeln!(&mut file,"{}",input_string)?;
    Ok(())

}

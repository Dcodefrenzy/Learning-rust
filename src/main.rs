#[derive(Debug)]
struct Person {
  name: String, 
  age:u32,
  gender:char,
  phoneNumber:u64
}

fn main() {
  let papa = Person {
    name: "Ayodeji Fakunle".to_string(), 
    age:25,
    gender:'M',
    phoneNumber: 09133475878
    
  };

  let Person {
    name,
    age,
    gender,
    phoneNumber
  } = papa;

println!("{}, age {}, gender {}", name, age, gender, phoneNumber);
}
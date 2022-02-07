#[derive(Debug)]
struct Person {
  name: String, 
  age:u32,
  gender:char
}

fn main() {
  let papa = Person {
    name: "Ayodeji Fakunle".to_string(), 
    age:25,
    gender:'M'
  };

  let Person {
    name,
    age,
    gender
  } = papa;

println!("{}, age {}, gender {}", name, age, gender);
}
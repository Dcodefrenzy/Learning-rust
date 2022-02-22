
struct Person {
  name: String, 
  age:u32,
  gender:char,
}

trait CallPerson {
  fn callName(&self)
}

impl CallPerson for Person {
  fn callName(&self) {
    println!("{}" &self.name);
  }
}
  
fn main() {
  let papa = Person {
    name: "Ayodeji Fakunle".to_string(), 
    age:25,
    gender:'M',
  }
    papa.CallName();
};



trait Human {
  fn greet(&self);
}

struct Student {
  name: &'static str,
  department: &'static str,
}

impl Human for Student {
  fn greet(&self) {
    println!(
      "{}",
      format!(
        "Hello! I'm {name}, in {dep}.",
        name = self.name,
        dep = self.department
      )
    );
  }
}

fn main() {
  let person = Student {
    name: "John",
    department: "science",
  };
  // let person = Student {
  //   name: "John".to_string(),
  //   department: "science".to_string(),
  // };
  person.greet();
  println!("{}", person.name);
}

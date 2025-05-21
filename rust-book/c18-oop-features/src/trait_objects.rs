// Hypothetical library traits and types
trait Draw {
  fn print_name(&self);
  fn draw(&self);
}

struct Screen {
  pub components: Vec<Box<dyn Draw>>
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.print_name();
      component.draw();
      println!("");
    }
  }
}

// Hypothetical user's self-defined types
struct Button {
  heigth: u8,
  label: String,
  width: u8
}

impl Draw for Button {
  fn print_name(&self) {
    println!("Button");
  }

  fn draw(&self) {
    let row = "*".repeat(usize::from(self.width));

    println!("{row}");
    println!("* {}", self.label);
    if self.heigth > 1 {
      for _ in 0..(self.heigth-1) {
        println!("*");
      }
    }
    println!("{row}");
  }
}

struct SelectItem {
  checked: bool,
  key: String
}

struct SelectBox {
  items: Vec<SelectItem>
}

impl Draw for SelectBox {
  fn print_name(&self) {
    println!("SelectBox");
  }

  fn draw(&self) {
    for select_item in self.items.iter() {
      let check_symbol = if select_item.checked { String::from("(X)") } else { String::from("( )") };
      println!("{check_symbol} {}", select_item.key);
    }
  }
}

pub fn gui_example() {
  println!("## Hypothetical GUI library used to draw different UI items, using a Vec<Box<dyn Draw>>");
  let screen = Screen {
    components: vec![
      Box::new(Button {
        label: String::from("click me"),
        heigth: 2,
        width: 15
      }),
      Box::new(SelectBox {
        items: vec![
          SelectItem { checked: false, key: String::from("apples") },
          SelectItem { checked: true, key: String::from("eggs") },
          SelectItem { checked: false, key: String::from("soap") }
        ]
      })
    ]
  };
  screen.run();
}
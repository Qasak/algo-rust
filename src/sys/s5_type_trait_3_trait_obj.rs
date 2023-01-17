// 子类型多态：静态分派与动态分派
struct Cat;
struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

// fn name<T: Animal>(animal: T) -> &'static str {
//     animal.name()
// }

fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct MarkdownFormatter;
impl Formatter for MarkdownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Markdown formatter");
        true
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Rust formatter");
        true
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with HTML formatter");
        true
    }
}

// pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
//     for formatter in formatters {
//         formatter.format(input);
//     }
// }

pub fn format(input: &mut String, formatters: Vec<Box<dyn Formatter>>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

// 这种泛型函数会根据具体使用的类型被单态化，编译成多个实例，是静态分派。
#[test]
fn test_static() {
    let cat = Cat;
    println!("{}", name(cat));
}


// 在运行时，构造一个 Formatter 的列表
// Trait Object 的底层逻辑就是胖指针。其中，一个指针指向数据本身，另一个则指向虚函数表
#[test]
fn test_dynamic() {
    let mut text = "Hello world!".to_string();
    // let html: &dyn Formatter = &HtmlFormatter;
    // let rust: &dyn Formatter = &RustFormatter;

    let html: Box<dyn Formatter> = Box::new(HtmlFormatter);
    let rust: Box<dyn Formatter> = Box::new(RustFormatter);
    let formatters = vec![html, rust];
    format(&mut text, formatters);

    println!("text: {}", text);
}
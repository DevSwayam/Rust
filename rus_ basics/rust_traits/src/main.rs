trait PrintFunction {
    fn print_string(&self) -> String;
    fn test_printer(&self) {
        println!("Its working");
    }
}

struct User {
    first_name: String,
    last_name: String,
}

impl PrintFunction for User {
    fn print_string(&self) -> String {
        return format!(
            "My first name is {} and my last name is {}",
            self.first_name, self.last_name
        );
    }
}

struct SimpleSummary {
    title: String,
}

trait Summary {
    fn summarise(&self) -> String {
        return String::from("Summarised");
    }
    fn summarise_title(&self);
}
impl Summary for SimpleSummary {
    fn summarise_title(&self) {
        println!("Summarised title is : {:?}", self.title);
    }
}

impl PrintFunction for SimpleSummary {
    fn print_string(&self) -> String {
        return format!("My Title is {} ", self.title);
    }
}

fn only_summarisers(u: &impl Summary) {
    u.summarise();
}

fn only_summarisers_with_generic<T: Summary + PrintFunction>(u: &T) {
    u.summarise();
}

fn main() {
    let user = User {
        first_name: String::from("Swayam"),
        last_name: String::from("Karle"),
    };

    println!("{:?}", user.print_string());
    user.test_printer();

    let summary = SimpleSummary {
        title: String::from("FHE"),
    };

    only_summarisers(&summary);
    summary.summarise_title();
    only_summarisers_with_generic(&summary);
}

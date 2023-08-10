trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.location)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        
    };
    
    println!("New article available! {}", article.summarize());
}

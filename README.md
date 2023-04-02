```rs
pub trait PassionateProgrammer {
    fn explore_new_tech(&self);
    fn enjoy_challenge(&self);
    fn consistently_push_limit(&self);
}
pub trait FastLearner {
    fn set_clear_goals(&self);
    fn learn_by_doing(&self);
    fn embrace_feedback(&self);
}
pub trait TeamPlayer {
    fn collaborate_with_teammates(&self);
}

struct Nrishinghananda<'a> {
    fullname: &'a str,
    username: &'a str,
    website: &'a str,
    primary_skills: Vec<&'a str>,
    secondary_skills: Vec<&'a str>,
    os: Vec<&'a str>,
    open_source_contributions: Vec<&'a str>,
}

impl<'a> PassionateProgrammer for Nrishinghananda<'a> {
    fn explore_new_tech(&self) {
        println!("{} is exploring new technology", self.fullname);
    }
    fn enjoy_challenge(&self) {
        println!("{} enjoys a good challenge", self.fullname);
    }
    fn consistently_push_limit(&self) {
        println!("{} is consistently pushing his limits", self.fullname);
    }
}

impl<'a> FastLearner for Nrishinghananda<'a> {
    fn set_clear_goals(&self) {
        println!("{} sets clear learning goals", self.fullname);
    }
    fn learn_by_doing(&self) {
        println!("{} learns by doing", self.fullname);
    }
    fn embrace_feedback(&self) {
        println!("{} embraces feedback as a learning opportunity", self.fullname);
    }
}

impl<'a> TeamPlayer for Nrishinghananda<'a> {
    fn collaborate_with_teammates(&self) {
        println!("{} collaborates well with his teammates", self.fullname);
    }
}

fn main() {
    let roynrishingha = Nrishinghananda {
        fullname : "Nrishinghananda Roy",
        username : "roynrishingha",
        website : "roynrishingha.com",
        primary_skills : vec!["Rust", "Actix-Web", "tokio", "axum", "tracing", "SurrealDB", "PostgreSQL", "REST API", "gRPC", "Substrate"],
        secondary_skills : vec!["Algorithms","Data Structures", "Python", "Github Action", "Docker", "Typescript", "GraphQL", "Debugging", "Software Documentation"],
        os : vec!["M1 MacOS", "PopOS", "Fedora"],
        open_source_contributions : vec!["rust-lang/rust-clippy", "google/tarpc"],
    };
}
```

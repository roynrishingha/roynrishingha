#![allow(dead_code, unused_variables)]

macro_rules! impl_traits {
    ($name:ident { $($field:ident : $type:ty),* $(,)? }) => {
        impl<'a> PassionateProgrammer for $name<'a> {
            fn enjoy_challenge(&self) {
                println!("{} enjoys a good challenge", self.name);
            }
            fn consistently_push_limit(&self) {
                println!("{} is consistently pushing his limits", self.name);
            }
        }

        impl<'a> FastLearner for $name<'a> {
            fn learn_by_doing(&self) {
                println!("{} learns by doing", self.name);
            }
            fn embrace_feedback(&self) {
                println!("{} embraces feedback as a learning opportunity", self.name);
            }
        }

        impl<'a> TeamPlayer for $name<'a> {
            fn collaborate_with_teammates(&self) {
                println!("{} collaborates well with his teammates", self.name);
            }
        }
    };
}

struct Nrishinghananda<'a> {
    name: &'a str,
    core_skills: Vec<&'a str>,
    infra_skills: Vec<&'a str>,
    basic_level_skills: Vec<&'a str>,
    website: &'a str,
}

impl_traits!(Nrishinghananda {
    name: &'a str,
    core_skills: Vec<&'a str>,
    infra_skills: Vec<&'a str>,
    basic_level_skills: Vec<&'a str>,
    website: &'a str,
});

fn main() {
    let roynrishingha = Nrishinghananda {
        name: "Nrishinghananda Roy",
        core_skills: vec![
            "Rust",
            "actix-web",
            "tokio",
            "tracing",
            "tonic",
            "REST API",
            "gRPC",
            "PostgreSQL",
            "Software Documentation",
            "Algorithms",
            "Data Structures",
        ],
        infra_skills: vec!["Github Actions", "Docker"],
        basic_level_skills: vec!["Python", "Typescript", "Vue.js", "Substrate"],
        website: "roynrishingha.com",
    };
}

pub trait PassionateProgrammer {
    fn enjoy_challenge(&self);
    fn consistently_push_limit(&self);
}

pub trait FastLearner {
    fn learn_by_doing(&self);
    fn embrace_feedback(&self);
}

pub trait TeamPlayer {
    fn collaborate_with_teammates(&self);
}

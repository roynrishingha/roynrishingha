macro_rules! impl_traits {
    ($name:ident { $($field:ident : $type:ty),* $(,)? }) => {
        impl<'a> FastLearner for $name<'a> {
            fn consistently_push_limit(&self) {
                println!("{} is consistently pushing his limits", self.name);
            }
            fn embrace_feedback(&self) {
                println!("{} embraces feedback as a learning opportunity", self.name);
            }
        }

        impl<'a> TeamPlayer for $name<'a> {
            fn collaborate_with_teammates(&self) {
                println!("{} collaborates effectively with his teammates", self.name);
            }
        }
    };
}

struct Nrishinghananda<'a> {
    name: &'a str,
}

impl_traits!(Nrishinghananda {
    name: &'a str,
});

fn main() {
    let roynrishingha = Nrishinghananda {
        name: "Nrishinghananda Roy",
    };
}

trait FastLearner {
    fn consistently_push_limit(&self);
    fn embrace_feedback(&self);
}

trait TeamPlayer {
    fn collaborate_with_teammates(&self);
}

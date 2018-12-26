extern crate rand;
use puppywhy::rand::distributions::{Distribution, Weighted, WeightedChoice};

macro_rules! weighted_choice_fn {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut items: Vec<Weighted<fn() -> String>> = vec![
                $(
                    Weighted {weight: $x, item: $y},
                    )*
            ];
            let wc = WeightedChoice::new(&mut items);
            let mut rng = rand::thread_rng();
            wc.sample(&mut rng)
        }
    };
}

macro_rules! weighted_choice {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut items = vec![
                $(
                    Weighted {weight: $x, item: $y},
                    )*
            ];
            let wc = WeightedChoice::new(&mut items);
            let mut rng = rand::thread_rng();
            wc.sample(&mut rng)
        }
    };
}

pub fn why() -> String {
    weighted_choice_fn!((1, special), (4, phrase), (6, sentence))()
}

fn special() -> String {
    weighted_choice!(
        (1, "why not?"),
        (1, "woof woof!"),
        (1, "why indeed?"),
        (
            1,
            "THERE IS AS YET INSUFFICIENT DATA FOR A MEANINGFUL ANSWER"
        ),
        (1, "life is full of mysteries"),
        (1, "I'm not telling you"),
        (1, "you know why")
    )
    .to_string()
}

fn phrase() -> String {
    weighted_choice_fn!(
        (1, || -> String {
            format!("for the {} {}", nouned_verb(), prepositional_phrase())
        }),
        (1, || -> String { format!("because {}", sentence()) }),
        (1, || -> String {
            format!("so as to {} {}", present_verb_phrase(), object())
        }),
        (1, || -> String {
            format!("to {} {}", present_verb_phrase(), object())
        })
    )()
}

fn preposition() -> String {
    weighted_choice!((1, "of"), (1, "from")).to_string()
}

fn prepositional_phrase() -> String {
    weighted_choice_fn!(
        (1, || -> String {
            format!("{} {} {}", preposition(), article(), noun_phrase())
        }),
        (1, || -> String {
            format!("{} {}", preposition(), proper_noun())
        }),
        (1, || -> String {
            format!("{} {}", preposition(), accusative_pronoun())
        })
    )()
}

fn sentence() -> String {
    format!("{} {}", subject(), predicate())
}

fn subject() -> String {
    weighted_choice_fn!(
        (1, proper_noun),
        (1, nominative_pronoun),
        (1, || -> String {
            format!("{} {}", article(), noun_phrase())
        })
    )()
}

fn proper_noun() -> String {
    weighted_choice!(
        (10, "Purple Puppy"),
        (3, "Donald Trump"),
        (5, "Woofer"),
        (2, "Purple Puppies' Porpoise"),
        (1, "Blue Puppy"),
        (1, "Red Puppy"),
        (1, "Green Puppy"),
        (1, "Yellow Puppy")
    )
    .to_string()
}

fn noun_phrase() -> String {
    weighted_choice_fn!(
        (3, noun),
        (3, || -> String {
            format!("{} {}", adjective_phrase(), noun_phrase())
        }),
        (1, || -> String {
            format!("{} and {}", noun_phrase(), noun_phrase())
        })
    )()
}

fn noun() -> String {
    weighted_choice!(
        (1, "puppy"),
        (1, "cat"),
        (1, "kitten"),
        (1, "dog"),
        (1, "stalker"),
        (1, "siege tank"),
        (1, "marine"),
        (1, "marauder"),
        (1, "zealot"),
        (1, "zergling"),
        (1, "baneling"),
        (1, "roach"),
        (1, "queen"),
        (1, "hydralisk"),
        (1, "ultralisk"),
        (1, "adept"),
        (1, "immortal"),
        (1, "sentry"),
        (1, "high templar"),
        (1, "dark templar"),
        (1, "archon"),
        (1, "liberator"),
        (1, "raven"),
        (1, "banshee"),
        (1, "viking"),
        (1, "battlecruiser"),
        (1, "phoenix"),
        (1, "void ray"),
        (1, "carrier"),
        (1, "tempest"),
        (1, "oracle"),
        (1, "mutalisk"),
        (1, "viper"),
        (1, "corruptor"),
        (1, "brood lord"),
        (1, "overlord"),
        (1, "overseer"),
        (1, "pupper")
    )
    .to_string()
}

fn nominative_pronoun() -> String {
    weighted_choice!(
        (1, "I"),
        (1, "you"),
        (1, "he"),
        (1, "she"),
        (1, "they"),
        (1, "we")
    )
    .to_string()
}

fn accusative_pronoun() -> String {
    weighted_choice!(
        (1, "me"),
        (1, "everyone"),
        (1, "her"),
        (1, "him"),
        (1, "them"),
        (1, "us")
    )
    .to_string()
}

fn nouned_verb() -> String {
    weighted_choice!(
        (1, "love"),
        (1, "approval"),
        (1, "satisfaction"),
        (1, "affection")
    )
    .to_string()
}

fn adjective_phrase() -> String {
    weighted_choice_fn!(
        (5, adjective),
        (1, || -> String {
            format!("{} and {}", adjective_phrase(), adjective_phrase())
        }),
        (3, || -> String {
            format!("{} {}", intensifier(), adjective())
        })
    )()
}

fn pos_intensifier() -> String {
    weighted_choice!(
        (1, "very"),
        (1, "really"),
        (1, "somewhat"),
        (1, "moderately"),
        (1, "mildly"),
        (1, "quite")
    )
    .to_string()
}

fn intensifier() -> String {
    weighted_choice_fn!(
        (2, pos_intensifier),
        (1, || -> String { format!("not {}", pos_intensifier()) })
    )()
}

fn adjective() -> String {
    weighted_choice!(
        (1, "purple"),
        (1, "green"),
        (1, "orange"),
        (1, "red"),
        (1, "blue"),
        (1, "yellow"),
        (1, "pink"),
        (1, "ultraviolet"),
        (1, "infrared"),
        (1, "spotted"),
        (1, "fluffy"),
        (1, "adorable"),
        (1, "terrified"),
        (1, "excited"),
        (1, "acceptable"),
        (1, "catlike"),
        (1, "doglike"),
        (1, "playful"),
        (1, "friendly"),
        (1, "spiky"),
        (1, "pointy"),
        (1, "aerodynamic"),
        (1, "checkered"),
        (1, "mottled"),
        (1, "two-dimensional"),
        (1, "tetrahedral"),
        (1, "triangular"),
        (1, "aggressive"),
        (1, "spherical"),
        (1, "cute"),
        (1, "African American"),
        (1, "differentiable"),
        (1, "open-source"),
        (1, "agreeable"),
        (1, "disagreeable"),
        (1, "tubular"),
        (1, "simply connected")
    )
    .to_string()
}

fn article() -> String {
    weighted_choice!((1, "the"), (1, "some"), (1, "a")).to_string()
}

fn predicate() -> String {
    weighted_choice_fn!(
        (1, || -> String {
            format!("{} {}", transitive_verb(), object())
        }),
        (1, intransitive_verb)
    )()
}

fn present_verb() -> String {
    weighted_choice!(
        (1, "defeat"),
        (1, "counter"),
        (1, "neutralise"),
        (1, "fool"),
        (1, "please"),
        (1, "satisfy"),
        (1, "outwit")
    )
    .to_string()
}

fn present_verb_phrase() -> String {
    weighted_choice_fn!(
        (7, present_verb),
        (1, || -> String { format!("obtain {} from", object()) })
    )()
}

fn transitive_verb() -> String {
    weighted_choice!(
        (1, "threatened"),
        (1, "told"),
        (1, "asked"),
        (1, "begged"),
        (1, "helped"),
        (1, "obeyed"),
        (1, "commanded"),
        (1, "confessed to"),
        (1, "hugged"),
        (1, "betrayed"),
        (1, "attacked"),
        (1, "advanced upon")
    )
    .to_string()
}

fn intransitive_verb() -> String {
    weighted_choice!(
        (1, "insisted on it"),
        (1, "suggested it"),
        (1, "told me to"),
        (1, "wanted it"),
        (1, "knew it was a good idea"),
        (1, "had a good feeling about it"),
        (1, "demanded it be this way"),
        (1, "exploded")
    )
    .to_string()
}

fn object() -> String {
    weighted_choice_fn!(
        (1, accusative_pronoun),
        (1, || -> String {
            let n = noun_phrase();
            let a = article();
            if a == "a" {
                let c = n.chars().next().unwrap();
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => return format!("an {}", n),
                    _ => (),
                }
            }
            format!("{} {}", a, n)
        })
    )()
}

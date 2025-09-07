fn main() {
    let health_potion = Potion::new(mix);
    println!("Hello, world!");
}

enum Effect {
    Rejuvenating(i8),
    Toxic(i8),
    Luck(i8),
    Alacrity(i8),
    Dexterity(i8),
    Charm(i8),
    Bland,
}
struct Potion<'e> {
    value: i32,
    tags: [&'e Effect; 5],
    title: String,
}

impl<'e> Potion<'e> {
    pub fn new<F>(mix: [&'e Ingredient<F>; 5]) -> Potion<'e>
    where
        F: FnMut(&mut Potion),
    {
        let mut tags: [&'e Effect; 5] = [&Effect::Bland; 5]; // Has to be initialized. ugh.
        let mut title = String::new();
        let mut value = 0;
        mix.iter().enumerate().map(|(i, item)| {
            tags[i] = &item.effect;
            title += item.adj;
            value += 1;
        });

        Potion { value, tags, title }
    }
}

struct Ingredient<'a, F: FnMut(&mut Potion)> {
    effect: Effect,
    alter: F,
    title: &'a str,
    adj: &'a str,
    stock: u32,
}

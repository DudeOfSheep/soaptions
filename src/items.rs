pub mod items {
    pub enum Effect {
        Rejuvenating(i8),
        Toxic(i8),
        Luck(i8),
        Alacrity(i8),
        Dexterity(i8),
        Charm(i8),
        Bland,
    }

    pub struct Ingredient<'a, F: FnMut(&mut Potion)> {
        effect: Effect,
        alter: F,
        title: &'a str,
        adj: &'a str,
        stock: u32,
    }

    impl<'a, F: FnMut(&mut Potion)> Ingredient<'a, F> {
        pub fn new(effect: Effect, alter: F, title: &'a str, adj: &'a str) -> Ingredient<'a, F> {
            Ingredient {
                effect,
                alter,
                title,
                adj,
                stock: 0,
            }
        }

        /// Manual assignment for an Ingredient's stock value
        pub fn set_stock(&mut self, stock: u32) {
            self.stock = stock;
        }

        /// Updates the ingredients stock by the ingredients stock equation
        pub fn drop_stock(&mut self) {
            self.stock -= 1;
        }
    }

    pub struct Potion<'e> {
        value: i32,
        tags: [&'e Effect; 5],
        title: String,
    }

    impl<'e> Potion<'e> {
        pub fn from<F>(mix: [Option<&'e Ingredient<F>>; 5]) -> Potion<'e>
        where
            F: FnMut(&mut Potion),
        {
            let mut tags: [&'e Effect; 5] = [&Effect::Bland; 5]; // Has to be initialized. ugh.
            let mut title = String::new();
            let mut value = 0;

            for (i, item) in mix.iter().enumerate() {
                if let Some(n) = item {
                    tags[i] = &n.effect;
                    title += &n.adj;
                    value += 1;
                }
            }

            Potion { value, tags, title }
        }

        pub fn new<F>(value: i32, tags: [&'e Effect; 5], title: String) -> Potion<'e>
        where
            F: FnMut(&mut Potion),
        {
            Potion { value, tags, title }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn potion_mixing() {}
    }
}

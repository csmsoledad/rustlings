// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)



mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub(self) fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub(crate) fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

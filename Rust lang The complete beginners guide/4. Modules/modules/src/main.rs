mod player;

fn main() {
    player::play_movie("Cat.mp4");
    player::play_audio("Cat.mp3");

    clean::perform_clean();
    clean::files::clean_files();
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning HDD..");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files..");
        }
    }
}

mod block;
mod hash;
fn main() {
    let genesis_block = block::Block::new(
        0,
        String::from("816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7"),
        None,
        String::from("Genesis Block"),
        1728895903,
    );
}

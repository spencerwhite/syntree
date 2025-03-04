use anyhow::Result;
use syntree::print;

fn main() -> Result<()> {
    let tree = syntree::tree! {
        "root" => {
            "child1" => {
                "nested1" => {
                    ("token1", 4),
                    ("token2", 1),
                    ("token3", 5)
                },
                ("token4", 1),
                ("token5", 5)
            },
            "child2" => {
            }
        },
        "root2" => {}
    };

    for n in tree.first().into_iter().flat_map(|n| n.walk()) {
        dbg!(n.value());
    }

    print::print(std::io::stdout(), &tree)?;
    Ok(())
}

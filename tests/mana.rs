use mana_symbols::Mana;
use svg::node::element::SVG;

fn compare_snapshot(name: &str, svg: SVG) {
    use std::path::PathBuf;
    let mut settings = insta::Settings::clone_current();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to retrieve value of CARGO_MANOFEST_DIR.");

    let mut path = PathBuf::from(&root_dir);
    path.push("snapshots");

    settings.set_snapshot_path(path);

    settings.bind(|| {
        insta::assert_binary_snapshot!(
            name,
            svg.to_string().as_bytes().into_iter().cloned().collect()
        );
    });
}

pub fn test_render(name: &str, symbol: &str) {
    let m: Mana = symbol.parse().unwrap();

    let svg = m.as_svg();
    compare_snapshot(name, svg);
}

#[test]
fn blue() {
    test_render("u.svg", "U");
}

#[test]
fn hybrid() {
    test_render("u_b.svg", "U/B");
}

#[test]
fn hybrid_phyrexian() {
    test_render("r_g_p.svg", "R/G/P");
}

#[test]
fn colorless_hybrid() {
    test_render("c_w.svg", "C/W");
}

#[test]
fn colorless() {
    test_render("c.svg", "C");
}

#[test]
fn snow() {
    test_render("s.svg", "S");
}

#[test]
fn zero() {
    test_render("0.svg", "0");
}

#[test]
fn five() {
    test_render("5.svg", "5");
}

#[test]
fn twenty() {
    test_render("20.svg", "20");
}

#[test]
fn generic_hybrid() {
    test_render("two_g.svg", "2/G");
}

#[test]
fn blue_phyrexian() {
    test_render("u_p.svg", "U/P");
}

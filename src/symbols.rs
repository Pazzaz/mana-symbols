use svg::{
    Document,
    node::element::{Path, SVG, path::Data, tag::Type},
    parser::Event,
};

use crate::Color;

pub fn colorless_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/c.svg"));
    parse_svg(content)
}

pub fn phyrexian_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/p.svg"));
    parse_svg(content)
}

pub fn snow_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/s.svg"));
    parse_svg(content)
}

pub fn color_symbol(color: Color) -> SVG {
    let content = match color {
        Color::White => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/w.svg")),
        Color::Blue => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/u.svg")),
        Color::Black => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/b.svg")),
        Color::Red => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/r.svg")),
        Color::Green => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/g.svg")),
    };
    parse_svg(content)
}

/// Returns `None` if `n` is larger than 20
pub fn number_symbol(n: usize) -> Option<SVG> {
    let content = match n {
        0 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/0.svg")),
        1 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/1.svg")),
        2 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/2.svg")),
        3 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/3.svg")),
        4 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/4.svg")),
        5 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/5.svg")),
        6 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/6.svg")),
        7 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/7.svg")),
        8 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/8.svg")),
        9 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/9.svg")),
        10 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/10.svg")),
        11 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/11.svg")),
        12 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/12.svg")),
        13 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/13.svg")),
        14 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/14.svg")),
        15 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/15.svg")),
        16 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/16.svg")),
        17 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/17.svg")),
        18 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/18.svg")),
        19 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/19.svg")),
        20 => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/numbers/20.svg")),
        _ => return None,
    };
    Some(parse_svg(content))
}

pub fn x_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/x.svg"));
    parse_svg(content)
}

pub fn y_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/y.svg"));
    parse_svg(content)
}

pub fn z_symbol() -> SVG {
    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/z.svg"));
    parse_svg(content)
}

fn parse_svg(content: &str) -> SVG {
    let mut document = Document::new().set("viewBox", (0, 0, 32, 32));

    for event in svg::read(content).unwrap() {
        if let Event::Tag("path", Type::Empty, attributes)
        | Event::Tag("path", Type::Start, attributes) = event
        {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            let path = Path::new().set("d", data);
            document = document.add(path);
        }
    }

    document
}

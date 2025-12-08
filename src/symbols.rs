use svg::{
    Document,
    node::element::{Path, SVG, path::Data, tag::Type},
    parser::Event,
};

use crate::{Color, SVG_WIDTH};

/// We store each symbol as a seperate SVG file in "/symbols", but when
/// compiling we statically load them using `include_str!`.
macro_rules! include_symbol {
    ($e:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols/", $e))
    };
}

/// Every symbol has identical SVG containers
fn document() -> SVG {
    Document::new().set("viewBox", (0, 0, SVG_WIDTH, SVG_WIDTH))
}

pub fn colorless_symbol() -> SVG {
    parse_add(include_symbol!("c.svg"), document())
}

pub fn phyrexian_symbol() -> SVG {
    parse_add(include_symbol!("p.svg"), document())
}

pub fn snow_symbol() -> SVG {
    let content = include_symbol!("s.svg");
    let mut paths = get_paths(content);
    let mut inner_path = paths.next().unwrap();
    let mut outline_path = paths.next().unwrap();
    inner_path = inner_path.set("fill", "white");
    outline_path = outline_path.set("fill", "black");

    document().add(inner_path).add(outline_path)
}

pub fn color_symbol(color: Color) -> SVG {
    let content = match color {
        Color::White => include_symbol!("w.svg"),
        Color::Blue => include_symbol!("u.svg"),
        Color::Black => include_symbol!("b.svg"),
        Color::Red => include_symbol!("r.svg"),
        Color::Green => include_symbol!("g.svg"),
    };
    parse_add(content, document())
}

/// Returns `None` if `n` is larger than 20
pub fn number_symbol(n: usize) -> Option<SVG> {
    let content = match n {
        0 => include_symbol!("numbers/0.svg"),
        1 => include_symbol!("numbers/1.svg"),
        2 => include_symbol!("numbers/2.svg"),
        3 => include_symbol!("numbers/3.svg"),
        4 => include_symbol!("numbers/4.svg"),
        5 => include_symbol!("numbers/5.svg"),
        6 => include_symbol!("numbers/6.svg"),
        7 => include_symbol!("numbers/7.svg"),
        8 => include_symbol!("numbers/8.svg"),
        9 => include_symbol!("numbers/9.svg"),
        10 => include_symbol!("numbers/10.svg"),
        11 => include_symbol!("numbers/11.svg"),
        12 => include_symbol!("numbers/12.svg"),
        13 => include_symbol!("numbers/13.svg"),
        14 => include_symbol!("numbers/14.svg"),
        15 => include_symbol!("numbers/15.svg"),
        16 => include_symbol!("numbers/16.svg"),
        17 => include_symbol!("numbers/17.svg"),
        18 => include_symbol!("numbers/18.svg"),
        19 => include_symbol!("numbers/19.svg"),
        20 => include_symbol!("numbers/20.svg"),
        _ => return None,
    };
    Some(parse_add(content, document()))
}

pub fn x_symbol() -> SVG {
    parse_add(include_symbol!("x.svg"), document())
}

pub fn y_symbol() -> SVG {
    parse_add(include_symbol!("y.svg"), document())
}

pub fn z_symbol() -> SVG {
    parse_add(include_symbol!("z.svg"), document())
}

fn parse_add(content: &str, mut svg: SVG) -> SVG {
    for path in get_paths(content) {
        svg = svg.add(path)
    }

    svg
}

fn get_paths(content: &str) -> impl Iterator<Item = Path> {
    svg::read(content).unwrap().filter_map(|event| {
        if let Event::Tag("path", Type::Empty, attributes)
        | Event::Tag("path", Type::Start, attributes) = event
        {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            let path = Path::new().set("d", data);
            Some(path)
        } else {
            None
        }
    })
}

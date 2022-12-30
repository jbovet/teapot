use clap::{Arg, ArgAction, Command, value_parser};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use teapot::{HttpStatusCode, StatusCode, TeaPotApp};

fn main() {
    let app = TeaPotApp::new();
    let matches = Command::new("teapot")
        .arg(
            Arg::new("code")
                .short('c')
                .required(true)
                .num_args(0..)
                .value_parser(value_parser!(u16))
                .action(ArgAction::Set),
        )
        .get_matches(); // builds the instance of ArgMatches

    let values: Vec<u16> = matches
        .get_many("code")
        .expect("`http code`is required")
        .copied()
        .collect();

    let mut status_code_list: Vec<&HttpStatusCode> = Vec::new();
    let mut invalid_codes = Vec::new();

    for code in values {
        let sc = StatusCode::from(code);
        if let StatusCode::Unknown(code) = sc {
            invalid_codes.push(code);
            continue;
        }
        status_code_list.push(app.http_codes.codes.get(sc.as_ref()).unwrap());
    }

    let mut table = Table::new();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .load_preset(UTF8_FULL)
        .set_header(vec![
            "Code",
            "Name",
            "Description",
            "Supplementary",
            "Documentation Url",
        ]);

    for sc in status_code_list {
        table.add_row(vec![
            Cell::new(sc.code)
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new(sc.name.to_string())
                .add_attribute(Attribute::Bold)
                .fg(Color::Blue),
            Cell::new(sc.description.to_string()),
            Cell::new(sc.supplementary_description.to_string()),
            Cell::new(sc.documentation_url.to_string()),
        ]);
    }

    for invalid in invalid_codes {
        table.add_row(vec![
            Cell::new(invalid)
                .add_attribute(Attribute::Bold)
                .fg(Color::Red),
            Cell::new("Unknown status code")
                .add_attribute(Attribute::Bold)
                .fg(Color::Yellow),
        ]);
    }
    println!("{table}");
}

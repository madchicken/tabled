use table_to_html::{Alignment, Entity, HtmlTable, Margin, Padding};

use tabled::Table;
use testing_table::test_table;

test_table!(
    table_iter_fmt,
    HtmlTable::new([["123", "324", "zxc"], ["123", "324", "zxc"]]),
    "<table>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_fmt,
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]]))),
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_fmt_multiline,
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([
        ["1\n2\n3", "3\n2\n4", "z\nx\nc"],
        ["12\n3", "32\n4", "zx\nc"],
    ]))),
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                    <p>"
    "                        3"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        3"
    "                    </p>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                    <p>"
    "                        4"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        z"
    "                    </p>"
    "                    <p>"
    "                        x"
    "                    </p>"
    "                    <p>"
    "                        c"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        12"
    "                    </p>"
    "                    <p>"
    "                        3"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        32"
    "                    </p>"
    "                    <p>"
    "                        4"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zx"
    "                    </p>"
    "                    <p>"
    "                        c"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_padding_cell,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_padding(Entity::Cell(1, 1), Padding::filled(4));
        html
    },
    "<style>"
    "    table:has(thead) > tbody > :nth-child(1) > :nth-child(2), table:not(:has(thead)) > tbody > :nth-child(2) > :nth-child(2) {"
    "      padding-bottom: 4px;"
    "      padding-left: 4px;"
    "      padding-right: 4px;"
    "      padding-top: 4px;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_padding_row,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_padding(Entity::Row(1), Padding::filled(4));
        html
    },
    "<style>"
    "    tbody > :nth-child(2) > td, thead > :nth-child(2) > th {"
    "      padding-bottom: 4px;"
    "      padding-left: 4px;"
    "      padding-right: 4px;"
    "      padding-top: 4px;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_padding_column,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_padding(Entity::Column(1), Padding::filled(40));
        html
    },
    "<style>"
    "    tbody > tr > :nth-child(2), thead > tr > :nth-child(2) {"
    "      padding-bottom: 40px;"
    "      padding-left: 40px;"
    "      padding-right: 40px;"
    "      padding-top: 40px;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_padding_global,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_padding(Entity::Global, Padding::new(50, 30, 10, 0));
        html
    },
    "<style>"
    "    tbody > tr > td, thead > tr > th {"
    "      padding-bottom: 0px;"
    "      padding-left: 50px;"
    "      padding-right: 30px;"
    "      padding-top: 10px;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_alignment_left,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_alignment(Entity::Global, Alignment::left());
        html
    },
    "<style>"
    "    tbody > tr > td, thead > tr > th {"
    "      text-align: left;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_span_column,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_column_span((0, 0), 3);
        html
    },
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th colspan=\"3\">"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_span_row,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_row_span((1, 1), 2);
        html
    },
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td rowspan=\"2\">"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_span_row_and_span_col,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_row_span((2, 1), 2);
        html.set_column_span((2, 1), 2);
        html
    },
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td rowspan=\"2\" colspan=\"2\">"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_margin,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_margin(Margin::new(5, 4, 3, 2));
        html
    },
    "<style>"
    "    table {"
    "      margin-bottom: 2;"
    "      margin-left: 5;"
    "      margin-right: 4;"
    "      margin-top: 3;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

test_table!(
    table_border,
    {
        let mut html = HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder([["123", "324", "zxc"], ["123", "324", "zxc"]])));
        html.set_border(10);
        html
    },
    "<style>"
    "    table, th, td {"
    "      border: 10px solid;"
    "    }"
    "</style>"
    "<table>"
    "    <thead>"
    "        <tr>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        0"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        1"
    "                    </p>"
    "                </div>"
    "            </th>"
    "            <th>"
    "                <div>"
    "                    <p>"
    "                        2"
    "                    </p>"
    "                </div>"
    "            </th>"
    "        </tr>"
    "    </thead>"
    "    <tbody>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "        <tr>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        123"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        324"
    "                    </p>"
    "                </div>"
    "            </td>"
    "            <td>"
    "                <div>"
    "                    <p>"
    "                        zxc"
    "                    </p>"
    "                </div>"
    "            </td>"
    "        </tr>"
    "    </tbody>"
    "</table>"
);

use cursive::{
    traits::*,
    views::{
        Button, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, Panel, SelectView,
        TextView,
    },
};

fn main() -> Result<(), std::io::Error> {
    let mut app = cursive::default();

    app.add_layer(
        Dialog::new()
            .title("Advanced Airline Seating Systems®")
            .button("Ok", |s| s.quit())
            .content(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            // SEATING
                            .child(
                                Panel::new(PaddedView::lrtb(
                                    2,
                                    2,
                                    1,
                                    1,
                                    TextView::new(concat!(
                                        "   A  B  C  D\n",
                                        "1  _  _  _  _\n",
                                        "2  _  _  _  _\n",
                                        "3  _  _  _  _\n",
                                        "4  _  _  _  _\n",
                                        "5  _  _  _  _\n",
                                        "6  _  _  _  _\n",
                                        "7  _  _  _  _\n",
                                        "8  _  _  _  _\n",
                                        "9  _  _  _  _",
                                    )),
                                ))
                                .title("Map"),
                            )
                            // COSTS
                            .child(
                                Panel::new(PaddedView::lrtb(
                                    2,
                                    2,
                                    1,
                                    1,
                                    LinearLayout::vertical()
                                        .child(
                                            ListView::new()
                                                .child(
                                                    "Ticket Cost:   $",
                                                    EditView::new()
                                                        .max_content_width(4)
                                                        .with_name("ticket_cost")
                                                        .fixed_width(5),
                                                )
                                                .delimiter()
                                                .child(
                                                    "Bag Cost:      $",
                                                    EditView::new()
                                                        .max_content_width(4)
                                                        .with_name("bag_cost")
                                                        .fixed_width(5),
                                                )
                                                .delimiter()
                                                .child(
                                                    "Number of Bags:",
                                                    EditView::new()
                                                        .max_content_width(4)
                                                        .with_name("number_of_bags")
                                                        .fixed_width(5),
                                                )
                                                .delimiter(),
                                        )
                                        .child(
                                            TextView::new("Total Cost: $1575")
                                                .with_name("total_cost"),
                                        ),
                                ))
                                .title("Costs"),
                            ),
                    )
                    // PASSENGERS
                    .child(
                        Panel::new(
                            LinearLayout::vertical()
                                .child(TextView::new("Name                 FFID    Seat"))
                                .child(
                                    LinearLayout::horizontal()
                                        .child(
                                            EditView::new()
                                                .with_name("passenger_name")
                                                .fixed_width(20),
                                        )
                                        .child(TextView::new(" "))
                                        .child(
                                            EditView::new()
                                                .max_content_width(6)
                                                .fixed_width(7)
                                                .with_name("passenger_ffid"),
                                        )
                                        .child(TextView::new(" "))
                                        .child(
                                            SelectView::new()
                                                .popup()
                                                .item_str("1")
                                                .item_str("2")
                                                .item_str("3")
                                                .item_str("4")
                                                .item_str("5")
                                                .item_str("6")
                                                .item_str("7")
                                                .item_str("8")
                                                .item_str("9")
                                                .with_name("passenger_seat_number"),
                                        )
                                        .child(TextView::new(" "))
                                        .child(
                                            SelectView::new()
                                                .popup()
                                                .item_str("A")
                                                .item_str("B")
                                                .item_str("C")
                                                .item_str("D")
                                                .with_name("passenger_seat_letter"),
                                        )
                                        .child(TextView::new(" "))
                                        .child(
                                            Button::new("Unboard", |_| ())
                                                .with_name("passenger_remove_button"),
                                        ),
                                )
                                .scrollable(),
                        )
                        .title("Passengers"),
                    )
                    .child(Button::new("Board Passenger", |_| ()))
                    .child(DummyView)
                    .child(TextView::new("©1960s Fresh Airlines").center()),
            ),
    );

    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
    };

    app.try_run_with(backend_init)
}

extern crate tinyfiledialogs;

use tinyfiledialogs::{DefaultColorValue, MessageBox, Icon, BoxButton};

fn main() {
    let choice = tinyfiledialogs::message_box(MessageBox::YesNo, "hello", "yes or no?",
                                              // Icon
                                              Some(Icon::Question),
                                              // Default button
                                              Some(BoxButton::CancelNo));

    let user_input = tinyfiledialogs::input_box("Enter user name", "Username:", "");

    let save_file = tinyfiledialogs::save_file_dialog("Save", "password.txt");

    let open_file = tinyfiledialogs::open_file_dialog("Open", "password.txt", None);

    let folder = tinyfiledialogs::select_folder_dialog("Select folder", "");

    let color = tinyfiledialogs::color_chooser_dialog("Choose a Color", DefaultColorValue::Hex("#FF0000"));

    #[cfg(not(windows))]
    let list = tinyfiledialogs::list_dialog("Test Dialog",
                                            &["Id", "Name"],
                                            Some(&["471", "Donald Duck",
                                                "1143", "Chris P. Bacon",
                                                "6509", "Moon Doge"]));

    println!("Choice {:?}", choice);
    println!("User input {:?}", user_input);
    println!("Save file {:?}", save_file);
    println!("Open file {:?}", open_file);
    println!("folder {:?}", folder);
    println!("color {:?}", color);

    #[cfg(not(windows))]
    println!("List {:?}", list);
}

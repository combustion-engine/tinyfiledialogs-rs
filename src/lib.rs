//! Rust bindings to the [tinyfiledialogs](https://sourceforge.net/projects/tinyfiledialogs/) library
//!
//! Example usage:
//!
//! ```ignore
//! extern crate tinyfiledialogs as tfd;
//!
//! use tfd::{DefaultColorValue, MessageBox, Icon, BoxButton};
//!
//! fn main() {
//!     let choice = tfd::message_box(MessageBox::YesNo, "hello", "yes or no?",
//!                                   // Icon
//!                                   Some(Icon::Question),
//!                                   // Default button
//!                                   Some(BoxButton::CancelNo));
//!
//!     let user_input = tfd::input_box("Enter user name", "Username:", None);
//!
//!     let save_file = tfd::save_file_dialog("Save", "password.txt");
//!
//!     let open_file = tfd::open_file_dialog("Open", "password.txt", None);
//!
//!     let folder = tfd::select_folder_dialog("Select folder", "");
//!
//!     let color = tfd::color_chooser_dialog("Choose a Color", DefaultColorValue::Hex("#FF0000"));
//!
//!     #[cfg(not(windows))]
//!     let list = tfd::list_dialog("Test Dialog",
//!                                 &["Id", "Name"],
//!                                 Some(&["471", "Donald Duck",
//!                                     "1143", "Chris P. Bacon",
//!                                     "6509", "Moon Doge"]));
//!
//!     println!("Choice {:?}", choice);
//!     println!("User input {:?}", user_input);
//!     println!("Save file {:?}", save_file);
//!     println!("Open file {:?}", open_file);
//!     println!("folder {:?}", folder);
//!     println!("color {:?}", color);
//!
//!     #[cfg(not(windows))]
//!     println!("List {:?}", list);
//! }
//! ```

#![deny(missing_docs)]

extern crate libc;

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::ptr;

pub mod ffi;

/// Type of message box to display
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum MessageBox {
    /// Simple message box with only an `Ok` button
    Ok,
    /// Message box with the choice of `Ok` and `Cancel`
    OkCancel,
    /// Message box with the choice of `Yes` or `No`
    YesNo,
}

/// Generic icon to be displayed beside message box messages
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Icon {
    /// Info-style icon
    Info,
    /// Warning-style icon
    Warning,
    /// Error-style icon
    Error,
    /// Question-style icon
    Question,
}

/// Which button to use or which was clicked
#[repr(i32)]
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum BoxButton {
    /// Either the `Cancel` or `No` button
    CancelNo = 0,
    /// Either the `Ok` or `Yes` button
    OkYes = 1,
}

impl MessageBox {
    #[inline]
    fn to_str(&self) -> &'static str {
        match *self {
            MessageBox::Ok => "ok",
            MessageBox::OkCancel => "okcancel",
            MessageBox::YesNo => "yesno"
        }
    }
}

impl Icon {
    #[inline]
    fn to_str(&self) -> &'static str {
        match *self {
            Icon::Info => "info",
            Icon::Warning => "warning",
            Icon::Error => "error",
            Icon::Question => "question",
        }
    }
}

/// Displays a simple message box of the given kind, title, and message
///
/// Optionally, an icon type can be given, and the default selected button for the message box.
///
/// The returned value is which button was clicked
pub fn message_box(kind: MessageBox, title: &str, message: &str, icon: Option<Icon>, default_button: Option<BoxButton>) -> BoxButton {
    let message_box_title = CString::new(title).unwrap();
    let message_box_message = CString::new(message).unwrap();
    let message_box_type = CString::new(kind.to_str()).unwrap();
    let message_box_icon = CString::new(icon.unwrap_or(Icon::Info).to_str()).unwrap();

    let res = unsafe {
        ffi::tinyfd_messageBox(
            message_box_title.as_ptr(),
            message_box_message.as_ptr(),
            message_box_type.as_ptr(),
            message_box_icon.as_ptr(),
            default_button.unwrap_or(BoxButton::OkYes) as i32)
    };

    match res {
        0 => BoxButton::CancelNo,
        1 => BoxButton::OkYes,
        _ => unimplemented!()
    }
}

fn input_box_impl(title: &str, message: &str, default: Option<&str>) -> Option<String> {
    let input_box_title = CString::new(title).unwrap();
    let input_box_message = CString::new(message).unwrap();
    let input_box_default = default.map(|default| CString::new(default).unwrap());

    let c_input = unsafe {
        ffi::tinyfd_inputBox(input_box_title.as_ptr(),
                             input_box_message.as_ptr(),
                             input_box_default.map(|d| d.as_ptr()).unwrap_or(ptr::null()))
    };

    if !c_input.is_null() {
        unsafe { Some(CStr::from_ptr(c_input).to_string_lossy().into_owned()) }
    } else { None }
}

/// Display a simple input box with the given title, message and optionally a default value
pub fn input_box(title: &str, message: &str, default: Option<&str>) -> Option<String> {
    input_box_impl(title, message, default.or(Some("")))
}

/// Display a simple masked input box suitable for passwords, with the given title and message
pub fn password_box(title: &str, message: &str) -> Option<String> {
    input_box_impl(title, message, None)
}

fn save_file_dialog_impl(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<String> {
    let save_dialog_title = CString::new(title).unwrap();
    let save_dialog_path = CString::new(path).unwrap();
    let save_dialog_des = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns = filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
    let ptr_filter_patterns = filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let c_file_name = unsafe {
        ffi::tinyfd_saveFileDialog(
            save_dialog_title.as_ptr(),
            save_dialog_path.as_ptr(),
            ptr_filter_patterns.len() as c_int,
            ptr_filter_patterns.as_ptr(),
            save_dialog_des.as_ptr())
    };

    if !c_file_name.is_null() {
        unsafe { Some(CStr::from_ptr(c_file_name).to_string_lossy().into_owned()) }
    } else { None }
}

/// Display a save file dialog with support for filtering file patterns
pub fn save_file_dialog_with_filter(title: &str, path: &str, filter_patterns: &[&str], description: &str) -> Option<String> {
    save_file_dialog_impl(title, path, Some((filter_patterns, description)))
}

/// Display a save file dialog without file pattern filters
pub fn save_file_dialog(title: &str, path: &str) -> Option<String> {
    save_file_dialog_impl(title, path, None)
}

fn open_file_dialog_impl(title: &str, path: &str, filter: Option<(&[&str], &str)>, multi: bool) -> Option<Vec<String>> {
    let open_dialog_title = CString::new(title).unwrap();
    let open_dialog_path = CString::new(path).unwrap();
    let open_dialog_des = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns = filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
    let ptr_filter_patterns = filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let c_file_name = unsafe {
        ffi::tinyfd_openFileDialog(
            open_dialog_title.as_ptr(),
            open_dialog_path.as_ptr(),
            ptr_filter_patterns.len() as c_int,
            ptr_filter_patterns.as_ptr(),
            open_dialog_des.as_ptr(),
            multi as c_int)
    };

    if !c_file_name.is_null() {
        let result = unsafe { CStr::from_ptr(c_file_name).to_string_lossy().into_owned() };

        Some(if multi { result.split('|').map(|s| s.to_owned()).collect() } else { vec![result] })
    } else { None }
}

/// Display an open file dialog for a single file and optional file pattern filters
pub fn open_file_dialog(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<String> {
    open_file_dialog_impl(title, path, filter, false).and_then(|v| v.into_iter().next())
}

/// Display an open file dialog with support for multiple files at a time
pub fn open_file_dialog_multi(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<Vec<String>> {
    open_file_dialog_impl(title, path, filter, true)
}

/// Display a dialog for selecting filesystem folders
pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let select_folder_title = CString::new(title).unwrap();
    let select_folder_path = CString::new(path).unwrap();

    let folder = unsafe {
        ffi::tinyfd_selectFolderDialog(select_folder_title.as_ptr(), select_folder_path.as_ptr())
    };

    if !folder.is_null() {
        unsafe { Some(CStr::from_ptr(folder).to_string_lossy().into_owned()) }
    } else { None }
}

#[cfg(not(windows))]
fn list_dialog_impl(title: &str, columns: &[&str], cells: Option<&[&str]>) -> Option<String> {
    let list_dialog_title = CString::new(title).unwrap();

    if columns.is_empty() {
        return None;
    }

    let list_dialog_columns = columns.iter().map(|s| CString::new(*s).unwrap()).collect::<Vec<CString>>();

    let ptr_list_dialog_columns = list_dialog_columns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let list_dialog_cells = cells.map_or(vec![], |f| f.iter().map(|s| CString::new(*s).unwrap()).collect());

    let ptr_list_dialog_cells = list_dialog_cells.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    let dialog = unsafe {
        ffi::tinyfd_arrayDialog(list_dialog_title.as_ptr(),
                                list_dialog_columns.len() as c_int,
                                ptr_list_dialog_columns.as_ptr(),
                                (list_dialog_cells.len() / list_dialog_columns.len()) as c_int,
                                ptr_list_dialog_cells.as_ptr())
    };

    if !dialog.is_null() {
        unsafe { Some(CStr::from_ptr(dialog).to_string_lossy().into_owned()) }
    } else { None }
}

#[cfg(windows)]
fn list_dialog_impl(_title: &str, _columns: &[&str], _cells: Option<&[&str]>) -> Option<String> {
    unimplemented!()
}

/// Displays a list chooser dialog.
///
/// **NOT AVAILABLE ON WINDOWS**
#[inline]
pub fn list_dialog(title: &str, columns: &[&str], cells: Option<&[&str]>) -> Option<String> {
    list_dialog_impl(title, columns, cells)
}

/// Default value for the color chooser dialog
pub enum DefaultColorValue<'a> {
    /// Hex value as a string
    Hex(&'a str),
    /// RGB value as a triplet of `u8`s (8-bit unsigned integers)
    RGB(&'a [u8; 3]),
}

/// Displays the system color chooser dialog
pub fn color_chooser_dialog(title: &str, default: DefaultColorValue) -> Option<(String, [u8; 3])> {
    let color_title = CString::new(title).unwrap();

    let rubbish = [0, 0, 0];

    let (color_default_hex, color_default_rgb) = match default {
        DefaultColorValue::Hex(hex) => (Some(CString::new(hex).unwrap()), &rubbish),
        DefaultColorValue::RGB(rgb) => (None, rgb),
    };

    let mut color_result_rgb = [0, 0, 0];

    let result = unsafe {
        ffi::tinyfd_colorChooser(color_title.as_ptr(),
                                 color_default_hex.map_or(ptr::null(), |h| h.as_ptr()),
                                 color_default_rgb.as_ptr(),
                                 color_result_rgb.as_mut_ptr())
    };

    if !result.is_null() {
        unsafe { Some((CStr::from_ptr(result).to_string_lossy().into_owned(), color_result_rgb)) }
    } else { None }
}
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::ptr;

pub mod ffi;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageBox {
    Ok,
    OkCancel,
    YesNo,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Icon {
    Info,
    Warning,
    Error,
    Question,
}

#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BoxButton {
    CancelNo = 0,
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

pub fn input_box(title: &str, message: &str, default: &str) -> Option<String> {
    input_box_impl(title, message, Some(default))
}

pub fn password_box(title: &str, message: &str) -> Option<String> {
    input_box_impl(title, message, None)
}

fn save_file_dialog_impl(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<String> {
    let save_dialog_title = CString::new(title).unwrap();
    let save_dialog_path = CString::new(path).unwrap();
    let save_dialog_des = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns =
    filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
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

pub fn save_file_dialog_with_filter(title: &str, path: &str, filter_patterns: &[&str], description: &str) -> Option<String> {
    save_file_dialog_impl(title, path, Some((filter_patterns, description)))
}

pub fn save_file_dialog(title: &str, path: &str) -> Option<String> {
    save_file_dialog_impl(title, path, None)
}

fn open_file_dialog_impl(title: &str, path: &str, filter: Option<(&[&str], &str)>, multi: bool) -> Option<Vec<String>> {
    let open_dialog_title = CString::new(title).unwrap();
    let open_dialog_path = CString::new(path).unwrap();
    let open_dialog_des = CString::new(filter.map_or("", |f| f.1)).unwrap();

    let filter_patterns =
    filter.map_or(vec![], |f| f.0.iter().map(|s| CString::new(*s).unwrap()).collect());
    let ptr_filter_patterns =
    filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

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
        let result = unsafe {
            CStr::from_ptr(c_file_name).to_string_lossy().into_owned()
        };
        Some(if multi {
            result.split('|').map(|s| s.to_owned()).collect()
        } else {
            vec![result]
        })
    } else {
        None
    }
}

pub fn open_file_dialog(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<String> {
    open_file_dialog_impl(title, path, filter, false).and_then(|v| v.into_iter().next())
}

pub fn open_file_dialog_multi(title: &str, path: &str, filter: Option<(&[&str], &str)>) -> Option<Vec<String>> {
    open_file_dialog_impl(title, path, filter, true)
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let select_folder_title = CString::new(title).unwrap();
    let select_folder_path = CString::new(path).unwrap();

    let folder = unsafe {
        ffi::tinyfd_selectFolderDialog(select_folder_title.as_ptr(), select_folder_path.as_ptr())
    };

    if !folder.is_null() {
        unsafe {
            Some(CStr::from_ptr(folder).to_string_lossy().into_owned())
        }
    } else {
        None
    }
}

#[cfg(not(windows))]
pub fn list_dialog(title: &str, columns: &[&str], cells: Option<&[&str]>) -> Option<String> {
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
        unsafe {
            Some(CStr::from_ptr(dialog).to_string_lossy().into_owned())
        }
    } else {
        None
    }
}

#[cfg(windows)]
pub fn list_dialog(_title: &str, _columns: &[&str], _cells: Option<&[&str]>) -> Option<String> {
    unimplemented!()
}

pub enum DefaultColorValue<'a> {
    Hex(&'a str),
    RGB(&'a [u8; 3]),
}

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
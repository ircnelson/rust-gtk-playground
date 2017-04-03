extern crate gtk;

use gtk::prelude::*;
use gtk::{
    Builder, Button, Grid, Window, Entry
};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("sample_window.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("window").unwrap();

    let button1: Button = builder.get_object("button1").unwrap();

    button1.connect_clicked(move |_| {
        let entry1: Entry = builder.get_object("entry1").unwrap(); 
        
        match entry1.get_text() {
            Some(text) => println!("{}", text),
            None => println!("none")
        }

        entry1.set_text("");
        entry1.grab_focus();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

# BitBlink

This Project aims to create a Linux Mobile Application for synchronizing (Fossasia) LED Name Badges with a mobile device via Bluetooth(LE). 
Archetype is the companys Android App BadgeMagic: 
- https://github.com/fossasia/badge-magic-android
- https://github.com/fossasia/badge-magic-ios

Developed with:
- libadwaita (GTK4 for mobile devices)
- Bluetooth LE

Ressources:
- https://gtk-rs.org/gtk4-rs/stable/latest/book/
- https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
- https://docs.gtk.org/gtk4/visual_index.html
- https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/
- https://docs.rs/libadwaita/0.4.1/libadwaita/index.html
- https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/
- https://blog.devgenius.io/initial-setup-for-a-gtk4-app-with-libadwaita-in-rust-using-vscode-b6f8c127a75e
- https://github.com/deviceplug/btleplug
- https://github.com/bluez/bluer
- https://github.com/Taiko2k/GTK4PythonTutorial/blob/main/README.md (Written in Python, may be useful nevertheless)
- https://github.com/gtk-rs/gtk4-rs/tree/master/examples
- https://linuxphoneapps.org/frameworks/libadwaita/


GTK, short for GIMP-Toolkit, is a cross-platform open-source library for the creation of graphical user interfaces (GUI). As of today it is in version 4.10.4 (05.06.2023). It is written in the C programming language, but several other programming languages have bindings to the underlying API, e.g. C++ (gtkmm), Python (PyGTK), JavaScript (Gjs) or, used in this project, Rust with Gtk-rs. In the following we will talk about Gtk-rs, as the project is written in Rust, but could possibly be adapted to every other language with GTK binding. Another very convenient feature of GTK since version 3.0 is the possibility to use CSS for styling the application, which makes setting e.g. the font size or coloring parts differently easy, especially for someone with experience in web development. On top of GTK, the design language used in this project is Libadwaita, which is the default theme for the GNOME Shell and Phosh. It comes with its own color palette, icons and typography. Phosh itself is the default graphical shell for several mobile Linux distributions, e.g. Mobian or postmarketOS, which is used in this project.
The entry point of the application is the instantiation of the application-object, which is the central object of a GTK application, as the name implies. It is the overlying object for all further operations and handles the interaction with the operating system. A GTK application consists mainly of an application and one or several windows, each containing multiple widgets. A widget is the basic building block for the GUI, ranging from simple containers for more specialized widgets (e.g. "CenterBox"), to different buttons, switches or scales. With that, GTK applications are built in a modular fashion, such that widgets can be stacked next or upon each other and, depending on the type, even nested. Widgets are also highly structured in terms of object-orientation and inheritance. A concept, which pervades the instantiation of every widget or object, like the application object, is the builder pattern. This design pattern works as follows: Every object we'd like to instantiate has a corresponding builder object (e.g. "ApplicationBuilder" for Application), from where we concatenate the input parameters for our object, instead of handling them in a constructor. After setting all the necessary parameters the .build() function is called on the builder object, which returns the corresponding object, e.g. Application. This provides a more "verbal" or "verbose" instantiation of object.
Once we built our application 
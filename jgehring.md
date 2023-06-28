# Graphical User Interface

## GTK

GTK, short for GIMP-Toolkit, is a cross-platform open-source library for the creation of graphical user interfaces (
GUI). As of today it is in version ```4.10.4``` (05.06.2023). It is written in the C programming language, but several
other
programming languages have bindings to the underlying API, e.g. C++ (gtkmm), Python (PyGTK), JavaScript (Gjs) or, used
in this project, Rust with Gtk-rs. In the following we will talk about Gtk-rs, as the project is written in Rust, but
could possibly be adapted to every other language with GTK binding. Another very convenient feature of GTK since version
3.0 is the possibility to use CSS syntax and most of its keywords for styling the application, which makes setting e.g.
the font size or coloring parts differently easy, especially for someone with experience in web development. On top of
GTK, the design language used in this project is Libadwaita, which is the default theme for the GNOME Shell and Phosh.
It comes with its own color palette, icons and typography. Phosh itself is the default graphical shell for several
mobile Linux distributions, e.g. Mobian or postmarketOS, which is used in this project.

## Program entry point

The entry point of the application is the instantiation of the ```Application```-object, which is the central object of
a GTK
application, as the name implies. It is the overlying object for all further operations and handles the interaction with
the operating system. A GTK application consists mainly of an application and one or several windows, each containing
multiple widgets. A widget is the basic building block for the GUI, ranging from simple containers for more specialized
widgets (e.g. ```CenterBox```), to different buttons, switches or scales. With that, GTK applications are built in a
modular
fashion, such that widgets can be stacked next or upon each other and, depending on the type, even nested. Widgets are
also highly structured in terms of object-orientation and inheritance. A concept, which pervades the instantiation of
every widget or object, like the application object, is the builder pattern. This design pattern works as follows: Every
object we'd like to instantiate has a corresponding builder object (e.g. ```ApplicationBuilder```
for ```Application```), from
where we concatenate the input parameters for our object, instead of handling them in a constructor. After setting all
the necessary parameters the ```.build()``` function is called on the builder object, which returns the corresponding
object,
e.g. ```Application```. This provides a more "verbal" or "verbose" instantiation of object.
Once we built our application object, the load_css method is called and the CSS file for the project is connected to a
CSSProvider object, which handles the CSS parsing and connects it to GTK widget styling.
After that we call the ```show_window``` in the closure connected to activating the application, which is given by the
following command to run the application.
In the ```show_window``` method an ```ApplicationWindow``` is created in a separate module. An ```ApplicationWindow```
is a window
subclass, which is connected to the given application and thus can handle calls like quitting the application.
From there we create a Rust ```std::boxed::Box``` (not to be confused with ```gtk::Box```) of
our ```ApplicationWindow```, which is
roughly speaking a pointer with added
functionality. From there we call the ```build_ui``` method, where the building blocks of the UI are created, combined
and
shown. With ```box::leak``` we submit a static reference to the ```ApplicationWindow``` to the method. This function is
used, when
we intend to keep the data for the remaining lifetime of the program, which applies to our ```ApplicationWindow``` and
enables
updating the content the ```ApplicationWindow``` is supposed to show instead of creating a new,
updated ```ApplicationWindow```, as
GTK has no dedicated refresh or update method, which calls a corresponding ```build_ui``` or ```update_ui``` method
again (at
least based on the knowledge of the authors and GTK version at the time of writing this documentation).

## Structure of the User Interface

The user interface consists of two main parts and is based on the BadgeMagic Android App. The concept of the UI at the
time of submitting the project is as follows: On top we have a menu bar with the usual buttons for hide, full screen and
quit, as well as a menu button. When clicking the menu button a popover window with the list of saved messages is shown.
Every list entry has its own button for deleting the message or loading it into the editor, the second part of the UI.
The message list has to loosely communicate with the editor, as we have to update the list when we save or delete a
message, and we have to fill the editor with the data from the messages, when we press a ```Edit``` button.
In the editor part we have an input field on top, where the user can type in the text he or she wants to be displayed,
additionally, in the following grid the user can choose between ten different icons, which are added to the text message
as unicode emojis.
Followed by the entry section we have three buttons, with which the user can switch between three pages, one with a
scale for adjusting the speed of the message (```Speed```), one for toggling the invert, marquee and flash functionality
of
the badge (```Effects```) and one for setting the animation mode of the message (```Animations```). This is done by a
GTK ```ViewStack```.
At the bottom of the ```ApplicationWindow``` we have two buttons, one for saving the current, configured message and one
for
calling the Bluetooth functionality and thus sending the message to the LED badge.

## The build_ui method

The ```build_ui``` method is the central block of creating and managing the GUI. Here we create the different parts of
the UI
previously described and append them to the central "```content```" ```gtk::Box```-Widget, which adds them to the end of
the content
in
the orientation specified at the construction of the content object, here vertically. The different parts of the editor
component have their own method for creation, which return ```boxed::Box```-References again. On the one hand, they
return the
complete widget, which is appended to a ```gtk::Box```, which combines all these widgets before appending
them to ```content```. On the other hand, these methods return the input widget of these sections, e.g. the entry field.
The
reason for that is, that we have to get and set the values of these widgets dynamically as well as connecting click
events to the buttons, and that is best done directly
on the reference of the widget, without the extra route of looking for the corresponding input widget in the parent
widget. After we built our ```HeaderBar```, ```Entry```, Icon ```Grid```, ```ViewStack``` and buttons at the bottom of
the window, we connect
the buttons with the click event performed on them. This is done with passing a closure to the ```connect_clicked```
-method.
In the case of the Save-button we first set the button insensitive to prevent several clicks on the button while
performing
the save operation. Then we create the message from our input widget values and pass it to the storage object in the
```save_message```-method. Then we call the ```build_ui``` method again, which then refreshes the content we set in our
window as well
as the click callbacks to the relevant buttons. We also have the possibility to keep the state of the input with the
Option
parameters of the method. With that, we update the message list and display the newly saved message in the list.
We do the same for every delete button in the message list except for the precedent action, which is deleting the
message.
For the ```Transfer``` button we again set the button insensitive, build the message and pass it to the bluetooth module
to send the message
to the badge. A special characteristic here is the usage of MainContext and its function ```spawn_local```. With that,
we
create a separate thread for sending the message, because we need to operate asynchronously, a condition set by the
Bluetooth functionality. We also use the ```clone!```
macro, which creates strong references to the input widgets as well as the ```Transfer``` button. Here we don't have to
refresh the UI. Events such as clicking a button and their connected callbacks are handled by the
GTK ```Main Event Loop```, which operates on a single thread by default, and we don't want to block this thread while
waiting for the connection to be established, sending the message etc. Here, setting the ```Transfer``` button inactive
is helpful twice, because it prevents the user from spawning several threads.
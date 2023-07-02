# BitBlink

This Project aims to create a Linux Mobile Application for synchronizing (Fossasia) LED Name Badges with a mobile device via Bluetooth(LE).
Archetype is the companys Android App BadgeMagic:
- https://github.com/fossasia/badge-magic-android
- https://github.com/fossasia/badge-magic-ios

The app allows to send messages which contain different settings to badges.
You can send up to eight texts with different emojis. (currently only 1 message is possible)
In the app you can choose a speed and an animations for every text.
You can also decide if you want the text to have a marquee, a flashing effect and if you would like the text to be inverted.
After you created your message you can send it to the badge. It is also possible to save your badges, to load them again later.
The loaded badges can later be edited, deleted or send to the badge. Share your saved badges by simply exporting the .txt files out of the bitBlinkData folder and import it to the same folder on the other device.

The app is compatible with Linux, Linux Mobile and macOS. It works for all devices with Bluetooth 4.0 or higher. 


Developed with:
- libadwaita (GTK4 for mobile devices)
- Bluetooth LE

Resources:
- https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
- https://gtk-rs.org/gtk4-rs/stable/latest/book/
- https://github.com/Taiko2k/GTK4PythonTutorial/blob/main/README.md (Written in Python, may be useful nevertheless)
- https://github.com/gtk-rs/gtk4-rs/tree/master/examples
- https://docs.gtk.org/gtk4/visual_index.html
- https://linuxphoneapps.org/frameworks/libadwaita/
- https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/
- https://docs.rs/libadwaita/0.4.1/libadwaita/index.html
- https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/
- https://blog.devgenius.io/initial-setup-for-a-gtk4-app-with-libadwaita-in-rust-using-vscode-b6f8c127a75e
- https://github.com/deviceplug/btleplug
- http://nilhcem.com/iot/reverse-engineering-bluetooth-led-name-badge



The program is split in three main components: 
The **ui** where the user can send commands what should be done. 
The **storage** where messages can be saved old messages can be load, edited and deleted. 
The **bluetooth** component where the messages can be sent to the badge.
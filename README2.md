# BitBlink

This Project aims to create a Linux Mobile Application for synchronizing (Fossasia) LED Name Badges with a mobile device 
via Bluetooth(LE).
Archetype is the companys Android App BadgeMagic:
- https://github.com/fossasia/badge-magic-android
- https://github.com/fossasia/badge-magic-ios

BitBlink allows you to send messages containing different emojis, speeds, effects and animations to one or multiple 
bluetooth LED-badges simultaneously. The badges are able to receive up to eight different texts in one message. 
(currently the transfer of only one text is implemented) The app lets you choose a variety of different speeds and 
animations for every text (see **Bluetooth** for more information).
You can also decide if you'd like the text to be displayed with a marquee, a flashing and/or an inverted effect.
After creating your message you can send it to the badge using the "Transfer" button. It is also possible to save your 
badges, to use them again later. The loaded badges can be edited further, deleted or send to the badge. You can also 
share your saved badges by simply exporting the .txt files out of the "bitBlinkData" folder and importing them into the 
same folder on the receiving device. BitBlink is compatible with Linux, Linux Mobile and macOS. It works for all devices 
equipped with Bluetooth 4.0 or higher. We can't really tell which badges are compatible with the app as there is a high 
variety of badges on the market. For our tests we used the fossasia led-badge: https://fossasia.com/product/led-badge/
The app could also be compatible with other badges that support Bluetooth and have a height of 11 LEDs per column, but 
we haven't tested that yet.

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
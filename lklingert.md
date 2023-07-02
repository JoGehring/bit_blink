


# Add that all tests not to be executed single threaded to end version of ReadMe.md.


\
\
\
\
\
\
### **Storage:**
The saved badge configurations are stored as .txt-Files in the JSON-format in a folder called "bitBlinkData" inside the current
working directory of the application. The storage of the files is handled using a very simple struct called "Storage". 
Before using this struct is has to be initialized using the "build_storage()"-Function, which creates the storage directory. 

**JSON:** \
The serialization and deserialization is handled using the serde framework.

**JSON attributes:**\
"file_name" --> name of the .json file. It is automatically generated using the timestamp from of the current time from
the chrono crate\
"hex_strings" --> a list of the strings that are to be displayed on the badge. The texts are encoded as hex 
strings in order to be compatible with the saved badges of the "Badge Magic" android app. The badge is able to
receive up to 8 separate strings in a single bluetooth transfer. The frontend currently only allows for sending
a single string however the entire backend supports the transfer of multiple strings in one transfer to make 
further development easier. \
"inverted" --> a boolean list that determines if the corresponding string in the "hex_string" list is supposed to
be inverted when displayed on the badge. \
"flash" --> a boolean list that determines if the corresponding string in the "hex_string" list is supposed to
flash when displayed on the badge. \
"marquee" --> a boolean list that determines if the corresponding string in the "hex_string" list is supposed to
have the marquee-border when displayed on the badge. \
"speed" --> a list of strings that determines the speed of the corresponding string in the "hex_string" list when 
displayed on the badge.
"mode" --> a list of strings that determines the animation of the corresponding string in the "hex_string" list
when displayed on the badge. 


### **Converting the input String to a badge compatible message**
The bluetooth message consists of multiple lines of hexadecimal strings with each line having a length of 32 digits. Every 
two hexadecimal digits define the configuration of eight LED-Pixels of the badge display (for more information regarding 
the exact structure of the bluetooth message see )


**Starting points for further development:** \
The "utils.rs" file contains all kinds of methods for handling the creation of new messages as well their en- 
and decoding into the proper badge-compatible hex-string format. \
It also contains the "build_single_message_from_first_text_vec_of_given_messages" function which remains unused
as of now due to the time constrains of the project. \
The purpose of this function was to enable the transfer of multiple message strings as mentioned above. The 
idea was to choose a selection of message-texts from the list of all messages (i.e. by using a checkbox) in 
the frontend and then combining all those messages into a single message and sending it to the badge. The only 
component still missing for this feature is the ability to check the messages in the list of all messages.

 

# Bluetooth:
The Bluetooth connection to the badge is established with **btleplug** (version 0.10.5). Btleplug is a Bluetooth Low 
Energy (**BLE**) module library for rust. BLE is completely separated to classic Bluetooth and therefore  not compatible 
to it. It is integrated in Bluetooth 4.0. Because of the advantages in power consumption it is used in the badges.
Btleplug supports a variety of operating systems like Windows 10, macOS and Linux, which is why we decided to use it. 
At the current state BitBlink only supports **Linux** and **Linux Mobile**, but could be expanded for other 
operating systems. It also works for **macOS**, but some features won't be rendered perfectly.

## Build Message:
The messages which are sent to the badge follow a persistent **structure**. Every message can hold different texts. 
Every text has different settings. You can decide for every text if you want the bits to be inverted, flashing or if 
they should have a marquee. Also, you can choose a speed (0 – 7) and a mode (0 -  8) for every text. You can have up to 
8 different texts with settings in every message and the badge will always only show one message.

After deciding what the badge should show on it, the actual message which will be sent via Bluetooth is built. To do 
that we add together the information from above in the right structure. First we will build it together as a simple 
string that consist of hexadecimal code that contains the data as bytes. So two hexadecimal figures will be one byte.

Every Bluetooth message to the badge starts with the **package header** “77616E670000”. With that the badge knows what 
to do with the following data that is sent to it.

The next one byte (corresponding to the next two hexadecimal figures) consists of the data if the texts should be 
**flashing**. As mentioned above we can decide that for every text separately. If we want the first text to be flashing, 
we simply switch the standard “00” to “01”. If we want only the second one to be flashing, we change the standard “00” 
to “02”. For the third one we change the “00” to “04”. If we want the first three to be flashing, we just add all three 
together to “07”. If we want all 8 texts to be flashing, we get “FF” which is equal to the decimal number “255”.

The next two hexadecimal figures contain the data for the **marquee**. It works exactly the same as the flash.

Next, we can choose which **speed** and **mode** we want to use. Every text has one hexadecimal number to decide which 
speed we want to use for it. There are 8 different speeds from speed 1 which corresponds to “0” till speed 8 which 
corresponds to “7”.

Every text has one hexadecimal number to decide which mode we want to use for it. There are 9 different modes (0 - 8):\
•	0 --> leftToRight\
•	1 --> rightToLeft\
•	2 --> upToDown\
•	3 --> downToUp\
•	4 --> FixedMiddle\
•	5 --> FixedLeft\
•	6 --> picture animation\
•	7 --> curtain animation\
•	8 --> laser animation\
The hexadecimal number for the speed and for the mode are put together one by one for all eight messages. For empty 
texts we just use “00” for the speed and mode. The same applies to every other place which is used for more texts than 
we need. Now we got the first 32 hexadecimal numbers.

The next 32 are used to tell the badge the **length** of the different texts. The first size is indicated by the first 
four hexadecimal figures and the second size uses the second four and so on. Four hexadecimal numbers used together can 
hold a decimal number as high as 164. Therefore, a single text can have 65.535 digits.

The next 32 hexadecimal figures are used for a **timestamp** which can be used by the app or the badge to track errors 
and to track which messages have been sent most recently. The structure of it begins with twelve zeros. After that we 
will add the last two digits of the year, followed by the month, the day, the hour, the minute and the second each with 
two digits. And after that we fill the rest with another 8 zeros.

After that we have 32 zeros digits which are used as a **separator** between the settings of the texts and the 
actual **texts**. Here every letter consists of 88 bits. Every bit shows if the belonging light of the badge is turned on 
or off. For example, here are the 88 bits of the letter “A”:\
00000000\
00111000\
01101100\
11000110\
11111110\
11000110\
11000110\
11000110\
11000110\
11000110\
00000000

One letter is eight in length and eleven in height. We than translate the 88 bits to 22 hexadecimal numbers. After we 
have done that for all 8 texts, we need to make sure that the whole strings' length modulo 32 is zero. If that isn’t the 
case we add the amount of the missing length as zeros to the end of the string.

Lastly, we will separate the whole string after every 32 hexadecimal figures and convert those 32 hexadecimal digits to 
a vector with the datatype u8.

## Send message to badge:
As mentioned above we use btleplug for sending the message.
Firstly, we connect to the devices Bluetooth adapter. With this we can scan for devices. To find the badge the system 
has a look at the names. If the name contains “LSLED” it should be the badge we are looking for. After we found a badge, 
we try to connect it. Then we discover the services of the badge. The badge has several services which are identified 
by the characteristic uuids for different actions like the system version of the badge, the battery level and the one 
we are looking for the service "write". We use this one to write the message to the badge. For that we send 32 
hexadecimal figures per write request as a vector with the datatype u8 to the badge. After we have done this for the 
whole message, we disconnect the badge. With that the sending process is finished and the badge should show the new 
message on the display. 


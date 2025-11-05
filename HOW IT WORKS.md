# How It Works

This system allows you to control your PC from an Android phone, using it as a remote gamepad.

**System Architecture:**

1.  **Mobile App (Client):**
    *   Developed for Android using Tauri.
    *   Displays a virtual gamepad with buttons and joysticks on the screen.
    *   When any action is performed (button press, joystick movement), the app sends the data to the server in JSON format.

2.  **Virtual Gamepad (Agent on PC):**
    *   This program runs on the target PC and is also a client of the server.
    *   It constantly "listens" to the same broadcast channel.
    *   When it receives data from the server, it translates and feeds these commands into the system using the **ViGEm** driver. This driver creates a fully functional virtual gamepad in the operating system, which is recognized by games and applications.


3.  **Server (PC):**
    *   Has a WebSocket listener that accepts incoming connections from mobile apps.
    *   Uses a broadcast mechanism: each new client that connects "subscribes" to a common channel.
    *   Receives JSON messages from a client and relays them to all subscribed clients.


**Important Requirement:**
For the virtual gamepad to work on Windows, you **must first download and install the ViGEm Bus Driver** from the official website:  

**In Simple Terms:**
Your phone sends gamepad commands to a server. The server acts as a dispatcher, relaying these commands to everyone who is listening. The agent program on your PC receives these commands and "presses" the corresponding buttons on a virtual gamepad that Windows can see. Games perceive this as if a physical gamepad was connected to the computer.
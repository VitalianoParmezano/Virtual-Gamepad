const WebSocket = window.__TAURI__.websocket;
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// Global variables for application state
let devfield;
//let js_debug_textview;
let gamepadInitialized = false;
let statusElement;
let ws;

/**
 * Initializes the application when DOM is fully loaded
 * Sets up event listeners and initializes core components
 */
window.addEventListener("DOMContentLoaded", () => {
    // Initialize configuration page elements
    document.getElementById("openGamepadBtn").addEventListener("click", showGamepadPage);
    document.getElementById("backToConfigBtn").addEventListener("click", showConfigPage);

    statusElement = document.getElementById("status");

    // Set up WebSocket connection handler
    document.getElementById("connectBtn").addEventListener("click", connectWebSocket);
    
    /**
     * Listen for debug events from Rust backend
     * Updates debug text view with received messages
     */
    listen("js_debug", (event) => {
        if (js_debug_textview) {
            js_debug_textview.textContent = event.payload;
        }
    });

    /**
     * Listen for data sending events from Rust
     * Forwards data to WebSocket server if connected
     */
    listen("send_to_server", async (event) => {
        if (!isConnected()) { 
            return; 
        }
        
        let sending_string = event.payload;
        
        try {
            await ws.send(sending_string);
        } catch (error) {
            console.error("❌ Error sending to server:", error);
        }
    });
});

/**
 * Establishes WebSocket connection to specified address
 * Updates connection status based on result
 */
async function connectWebSocket() {
    const address = document.getElementById("wsAddress").value;
    
    if (!address) {
        statusElement.textContent = "❌ Please enter address";
        return;
    }

    try {
        statusElement.textContent = "⏳ Connecting...";
        ws = await WebSocket.connect(address);
        statusElement.textContent = "✅ Connected";
    } catch (error) {
        statusElement.textContent = "❌ Connection error";
    }
}

/**
 * Shows gamepad page and initializes gamepad controls
 * Only initializes gamepad once to avoid duplicate event listeners
 */
function showGamepadPage() {
    document.getElementById("configPage").style.display = "none";
    document.getElementById("gamepadPage").style.display = "block";
    
    // Initialize gamepad only on first opening
    if (!gamepadInitialized) {
        initGamepad();
        gamepadInitialized = true;
    }
    
    // Notify Rust backend that gamepad was opened
    invoke("on_gamepad_opened");
}

/**
 * Returns to configuration page
 * Hides gamepad and shows configuration interface
 */
function showConfigPage() {
    document.getElementById("gamepadPage").style.display = "none";
    document.getElementById("configPage").style.display = "flex";
}

/**
 * Initializes all gamepad components
 * Sets up joysticks, buttons, and triggers with touch event handlers
 */
function initGamepad() {
    // js_debug_textview = document.getElementById("js_debug");
    // devfield = document.getElementById("devField");

    // Small delay to ensure DOM is fully updated
    setTimeout(() => {
        createJoystick("stickLeft", "leftJoy");
        createJoystick("stickRight", "rightJoy");
        initButtons();
        initTriggers();
        console.log("🎮 Gamepad initialized");
    }, 100);
}

/**
 * Creates a touch-sensitive joystick controller
 * Handles touch events and calculates joystick position values
 */
function createJoystick(stickId, containerId) {
    const stick = document.getElementById(stickId);
    const container = document.getElementById(containerId);
    
    if (!stick || !container) {
        console.error(`❌ Element not found: ${stickId} or ${containerId}`);
        return;
    }
    
    setTimeout(() => {
        const rect = container.getBoundingClientRect();
        const center = { 
            x: rect.width / 2, 
            y: rect.height / 2 
        };
        const radius = rect.width / 2;
        
        // Store active touches for this joystick
        const activeTouches = new Map();

        console.log(`🎯 Joystick ${stickId} initialized:`, { center, radius });

        /**
         * Calculates stick position from touch coordinates
         * Constrains movement to joystick boundary
         */
        function getPosFromTouch(touch, containerRect) {
            let x = touch.clientX - containerRect.left - center.x;
            let y = touch.clientY - containerRect.top - center.y;

            let dist = Math.sqrt(x * x + y * y);
            if (dist > radius - stick.offsetWidth / 2) {
                x = (x * (radius - stick.offsetWidth / 2)) / dist;
                y = (y * (radius - stick.offsetHeight / 2)) / dist;
            }
            return { x, y };
        }

        /**
         * Updates stick visual position and sends data to backend
         * Normalizes coordinates to standard joystick range
         */
        function updateStick(touchId, x, y) {
            // Update stick visual position
            stick.style.transform = `translate(calc(-50% + ${x}px), calc(-50% + ${y}px))`;
            
            let normX = x / (radius - stick.offsetWidth / 2);
            let normY = y / (radius - stick.offsetHeight / 2);

            let joyX = Math.round(normX * 32767);
            let joyY = Math.round(normY * -32768);

            if (devfield) {
                devfield.textContent = `${stickId}: ${joyX}, ${joyY} (touch: ${touchId})`;
            }
            send_stick_data(stickId, joyX, joyY);
        }

        /**
         * Handles touch start events on joystick
         * Activates joystick control when touched within bounds
         */
        function handleTouchStart(e) {
            const containerRect = container.getBoundingClientRect();
            
            // Process all touches
            for (let touch of e.touches) {
                const dx = touch.clientX - containerRect.left - center.x;
                const dy = touch.clientY - containerRect.top - center.y;
                const dist = Math.sqrt(dx * dx + dy * dy);

                if (dist <= radius && !activeTouches.has(touch.identifier)) {
                    // Found new touch within this joystick
                    const pos = getPosFromTouch(touch, containerRect);
                    activeTouches.set(touch.identifier, pos);
                    updateStick(touch.identifier, pos.x, pos.y);
                    e.preventDefault();
                    break; // One joystick - one touch
                }
            }
        }

        /**
         * Handles touch move events
         * Updates joystick position based on finger movement
         */
        function handleTouchMove(e) {
            const containerRect = container.getBoundingClientRect();
            
            // Find active touch for this joystick
            for (let touch of e.touches) {
                if (activeTouches.has(touch.identifier)) {
                    const pos = getPosFromTouch(touch, containerRect);
                    activeTouches.set(touch.identifier, pos);
                    updateStick(touch.identifier, pos.x, pos.y);
                    e.preventDefault();
                    break;
                }
            }
        }

        /**
         * Handles touch end events
         * Resets joystick position when touch is released
         */
        function handleTouchEnd(e) {
            let shouldReset = false;
            
            // Check if active touch was removed
            for (let touch of e.changedTouches) {
                if (activeTouches.has(touch.identifier)) {
                    activeTouches.delete(touch.identifier);
                    shouldReset = true;
                }
            }
            
            // Reset if no active touches remain
            if (shouldReset && activeTouches.size === 0) {
                updateStick('reset', 0, 0);
            }
        }

        /**
         * Handles touch cancel events
         * Cleans up touch tracking when touch is cancelled
         */
        function handleTouchCancel(e) {
            // Clear all touches on cancel
            for (let touch of e.changedTouches) {
                activeTouches.delete(touch.identifier);
            }
            
            if (activeTouches.size === 0) {
                updateStick('cancel', 0, 0);
            }
        }

        // Add event listeners
        container.addEventListener("touchstart", handleTouchStart, { passive: false });
        container.addEventListener("touchmove", handleTouchMove, { passive: false });
        container.addEventListener("touchend", handleTouchEnd);
        container.addEventListener("touchcancel", handleTouchCancel);

    }, 50);
}

/**
 * Initializes all button elements with touch event handlers
 * Sends button press/release events to Rust backend
 */
function initButtons() {
    const buttons = document.querySelectorAll(".button");
    console.log(`🎯 Found ${buttons.length} buttons`);

    buttons.forEach((btn) => {
        // Add touch start handler with passive: false
        btn.addEventListener("touchstart", async (e) => {
            e.preventDefault();
            if (devfield) {
                devfield.textContent = `Btn: ${btn.id} pressed`;
            }
            try {
                await invoke("buttons_info", { btn: btn.id, status: true });
            } catch (err) {
                console.error("Button press error:", err);
            }
        }, { passive: false });

        // Add touch end handler with passive: false
        btn.addEventListener("touchend", async (e) => {
            e.preventDefault();
            if (devfield) {
                devfield.textContent = `Btn: ${btn.id} released`;
            }
            try {
                await invoke("buttons_info", { btn: btn.id, status: false });
            } catch (err) {
                console.error("Button release error:", err);
            }
        }, { passive: false });

        btn.addEventListener("touchcancel", async (e) => {
            e.preventDefault();
            if (devfield) {
                devfield.textContent = `Btn: ${btn.id} cancelled`;
            }
            try {
                await invoke("buttons_info", { btn: btn.id, status: false });
            } catch (err) {
                console.error("Button cancel error:", err);
            }
        });
    });
}

/**
 * Initializes trigger elements with touch event handlers
 * Sends analog trigger values to Rust backend
 */
function initTriggers() {
    const triggers = document.querySelectorAll('.trigger');
    console.log(`🎯 Found ${triggers.length} triggers`);

    triggers.forEach((trigger) => {
        trigger.addEventListener("touchstart", async (e) => {
            e.preventDefault();
            if (devfield) {
                devfield.textContent = `Trigger: ${trigger.id} pressed`;
            }
            try {
                // Send value 255 when pressed (full pressure)
                await invoke("trigger_info", { trigger: trigger.id, value: 255 });
            } catch (err) {
                console.error("Trigger press error:", err);
            }
        });

        trigger.addEventListener("touchend", async (e) => {
            e.preventDefault();
            if (devfield) {
                devfield.textContent = `Trigger: ${trigger.id} released`;
            }
            try {
                // Send value 0 when released (no pressure)
                await invoke("trigger_info", { trigger: trigger.id, value: 0 });
            } catch (err) {
                console.error("Trigger release error:", err);
            }
        });
    });
}

/**
 * Sends joystick position data to Rust backend
 * Converts coordinates to standard joystick range
 */
async function send_stick_data(stickName, x, y) {
    try {
        await invoke("stick_info", {
            stick: stickName,
            x: x,
            y: y,
        });
    } catch (err) {
        console.error("Error sending stick data:", err);
    }
}

/**
 * Debug function to check element availability
 * Logs all key gamepad elements to console
 */
function debugElements() {
    console.log("🔍 Debug elements:");
    console.log("stickLeft:", document.getElementById("stickLeft"));
    console.log("leftJoy:", document.getElementById("leftJoy"));
    console.log("stickRight:", document.getElementById("stickRight"));
    console.log("rightJoy:", document.getElementById("rightJoy"));
    console.log("devField:", document.getElementById("devField"));
    //console.log("js_debug:", document.getElementById("js_debug"));
}

/**
 * Checks if WebSocket connection is active and ready
 * @returns {boolean} True if connected and ready
 */
function isConnected() {
    return ws && ws.readyState === WebSocket.OPEN;
}
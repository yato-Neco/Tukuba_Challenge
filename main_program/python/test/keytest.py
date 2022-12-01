from pynput import keyboard

def on_press(key):
    print("in", key)

def on_release(key):
    print("out", key)

# Collect events until released
with keyboard.Listener(
        on_press=on_press,
        on_release=on_release) as listener:
    listener.join()


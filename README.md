Flatkvm is a tool to easily run [flatpak](https://flatpak.org/) apps isolated inside a VM, using QEMU/KVM.

flatkvm-paste can be used to send text to a Flatkvm session's clipboard.

# Usage

flatkvm-paste accepts as a single argument the PID of a **flatkvm** process and reads the data to be pasted from **stdin**. This way, flatkvm-paste can be easily integrated into scripts and/or associated to keybindings on window managers like [i3wm](https://i3wm.org/).

If you have flatkvm session running [gedit](https://flathub.org/apps/details/org.gnome.gedit), you can do something like this:

```
echo "Hello world!" | flatkvm-paste `pgrep -f "flatkvm run.*gedit"`
```

You can also use [xclip](https://github.com/astrand/xclip) (included on most distributions) to send the current clipboard contents:

```
xclip -o | flatkvm-paste `pgrep -f "flatkvm run.*gedit"`
```

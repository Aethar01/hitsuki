# hitsuki 日月
Hitsuki is a commandline tool for dynamic time based wallpapers in x11 based window managers. It is written in rust and uses the feh to set the background.

## Usage

In order to make a time of day based wallpaper, simply place your series of pictures into any folder like so:

```
wallpaperdirectory
└───folder1
│   │   1.png
│   │   2.png
│   │   12.jpeg
│   │   18.png
│   
└───folder2
    │   6.jpeg
    │   9.png
    │   12.png
    │   18.png
    │   21.png
```

With the number representing the hour in which you wish the wallpaper to be displayed. 

### Add wallpapers

Now add the folders to the hitsuki config with the add command.

```
hitsuki add /path/to/wallpaperdirectory/folder1
hitsuki add /path/to/wallpaperdirectory/folder2
````
---
### Set wallpaper

Now you can set the current active wallpaper using:

```
hitsuki set folder1
```
or
```
hitsuki -s folder1
```
---

### Start hitsuki

From here you are free to run:

```
hitsuki start
```
---

You can now cycle your wallpapers with:
```
hitsuki next
```
and
```
hitsuki prev
```
---

### Additional usage

If you use the -s option for set you can run a command afterwards eg.
```
hitsuki -s folder1 start
```

Use the -d option to start hitsuki as a daemon
```
hitsuki -d start
```

Use the -r option to restart the daemon if it is already running (Useful for putting in scripts and changing the wallpaper when daemonized)
```
hitsuki -dr start
```

Use the following to list all the current added directories
```
hitsuki -l
```

### Remove wallpaper

Finally if you wish to remove a wallpaper directory from the list:
```
hitsuki remove folder1
```

## Example

Let's say for example in a window manager config file:
```
hitsuki -drs folder1 start
```
This will start hitsuki using wallpaper1 as a daemon and if the config file is reloaded, the daemon is restarted.

# adbackup [![Build Status](https://travis-ci.org/DonatJR/adbackup.svg?branch=master)](https://travis-ci.org/DonatJR/adbackup) [![Build status](https://ci.appveyor.com/api/projects/status/la91b294jegvmejw?svg=true)](https://ci.appveyor.com/project/DonatJR/adbackup) [![Crates.io](https://img.shields.io/crates/v/adbackup.svg)](https://crates.io/crates/adbackup)

backup tool for android written in rust which can either be used as a module and as a cli-tool

`adbackup` uses something similar to the 
[auto backup mechanism](https://developer.android.com/guide/topics/data/autobackup.html) of android. Normally, if 
enabled, this mechanism is used, to backup the android system into the google cloud. With the `adbackup`-cli or the 
module it is possible to create backups of folders, apps or the complete system. 

## Setup
In this version of adbackup we have to do some initialisation work: Install `adbackup`, `adb`, add `adb` to the 
environment path and enable debug mode on the android device.

#### Computer
`adbackup` is using the [android debug bridge (adb)](https://developer.android.com/studio/command-line/adb.html) to 
communicate with android devices. Before using adbackup 
1. install adb from the official 
[android developer](https://developer.android.com/studio/releases/platform-tools.html#download) site.
1. Now add `adb` to your environment variable 
([tutorial](https://www.xda-developers.com/adb-fastboot-any-directory-windows-linux/)). 
1. [Download](https://github.com/DonatJR/adbackup/releases) and install `adbackup`.

The computer is ready to work with `adbackup`. To check if everything is well configured, type `adbackup devices`.

#### Android device
To bring the android device into a state, in which `adbackup` can be used, enable the debug mode:
Go to 
1. `Settings`
1. `About phone`
1. Press five times on the `Build number`
1. Go back and into the new `Developer options`
1. Activate at the top of the screen the developer options and enable `Android debugging` in the list below.

If the device is connected to the computer start another try with `adbackup devices`.

## Limitations
Every app developer can specify if it is possible to backup the app in general or just parts of it or can completely 
disable it, [here](https://developer.android.com/guide/topics/data/autobackup.html) you can 
read more about it. Because of that, it could be possible, that not every app on the device an be backed up. But until 
know, there is no better way to do android backups without rooting the device.  

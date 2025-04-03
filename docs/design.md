# DOSContainer overall design

The purpose of DOSContainer is to automate the boring work of retro computing
collection building. The DOS platform is a notoriously difficult beast to work
with because it grew along with the Personal Computer for a decade and a half,
surviving all this time by being endlessly configurable. While that flexibili-
ty was a quality of the platform at the time, it throws present-day collection
builders for quite a loop. Say you have a collection of 4000 games, of which
some 2500 use the CuteMouse driver to control the mouse.. and now the makers
of CuteMouse come out with a new version. You need to do three things:

  * Figure out which of the 2500 games in your collection use CuteMouse;
  * Figure out which of that subset need an update to the CuteMouse driver;
  * Actually update potentially hundreds of disk images with a new driver;

## Borrowing from Docker

Docker is a definitely 21st century technology that allow IT folks all around
the world to run and update software in a very automated way. What it essenti-
ally does, is stack layer upon layer of changes on top of each other to end up
with a desired end state. Every udpate entails the addition of another layer,
from which Docker builds a new final `container image'.

DOSContainer approaches the DOS platform in a similar way. As a collection
builder, you define your portfolio of games in terms of any number of layers.

  * Physical: a virtual disk and its low-level properties. Are we dealing
    with a floppy? What kind? How is it formatted?
  * Foundation: the absolute bare minimum of an operating system as needed
    to start up the computer but absolutely no more than that.
  * Any number of `middleware' layers: prerequisite software that the game
    or application needs to function. These can be operating system components
    or third-party utilities like memory management tools etc.
  * Application or game core: the files that belong to the core of your game
    or application. These are essentialy unchanging between releases of the
    game and are unaffected by configuration changes.
  * Application or game configuration layers: everything that's required to
    configure your game or application to fit a particular (hardware) envi-
    ronment.

Where Docker uses the Dockerfile to define what we want Docker to do, we
define a Manifest in DOSContainer to define a number of properties for the
games and applications in our collections. A Manifest in DOSContainer takes
the form of a TOML-formatted text file. TOML is a machine-readable file format
that's also easy for humans to read, write and understand. The goal for
DOSContainer is to be as simple as possible, so I moved from YAML to TOML for
ease of use.

## Cascading configuration

A DOSContainer collection may contain a ```config.toml``` file at the root of
the collection directory. This defines settings that will be used all the way
down into the entire collection, unless they are overridden by another direc-
tive either in a directory-level ```config.toml``` file or in the individual
Manifest file of a specific game or application.

In this way, configuration cascades. You can set things in very broad strokes
at the top of your collection, and refine things as you recurse into subdirec-
tories.

  * Set everything you can as high up in the collection as possible.
  * Use directories to override top-level settings.
  * Only use settings in individual Manifests if you absolutely have to.

## Manifest specification

Every DOSContainer Manifest file contains a number of fields that are required
for DOSContainer to function. First, the manifest itself:

  ```
  [manifest]
  version = 1
  ```
The manifest version is required, even though the only supported version at
this time is 1. It indicates wich version of the Manifest file format the
Manifest itself was written in. Future versions of DOSContainer may be able
to convert between versions and upgrade existing Manifests. In order to not
turn this into a raging mess, we start by versioning the format from the very
beginning.

  ```
  [physical]
  category = floppy
  type = F35_144
  format = F35_144
  filesystem = FAT12
  ```
The physical layer is defined by the `physical' block, of which only one may
exist in a Manifest. According to the rules of cascading, you can set this at
the very top of the collection so that DOSContainer will generate one type of
disk for your entire collection. Useful if you have a PC with a specific type
of drive that you want to use for all your games and applications.

You can differentiate by using year-based or technology-based subfolders so
that you may create 5.25" 160KB floppy images for anything from 1981, while
you choose 3.5" 1.44MB for games from 1990 instead. That'd be a matter of set-
ting your preferences once in the 1981 and 1990 folders, and DOSContainer will
take care of it from there. Whatever you set in your year folder will override
anything that was set at the top level of your collection.

So if you set 3.5" 720KB floppy as your disk at the top of the collection, it
would tell DOSContainer to generate 3.5" 720KB floppies for all of your col-
lection except for the contents of 1981 and 1990.

As for the fields supported in the ```physical``` section, we currently only
support floppies but the fields are already defined in the format to handle
hard disks later on.

  * category: floppy or hdd.
  * type: one of the fixed floppy types, or a hard disk type.
  * format: how to format the disk. Omit if you want to use the disk optimally.
  * filesystem: omit to let the OS decide, or choose FAT12, FAT16, FAT16B,
    VFAT or FAT32. Pick unsupported combinations at your own peril here!
  * cylinders: Only applicable for hard disks, denotes the number of cylinders.
  * heads: Only applicable for hard disks, denotes the number of heads.
  * sectors: Only applicable for hard disks, denotes the number of sectors per
    track.

### Floppy disk types

DOSContainer supports generating any floppy disk type that IBM PC-DOS or MS-
DOS ever officially supported. The values in the following table are valid in
any manifest that has the ```category``` field set to ```floppy```.

| Value    |    Physical disk equivalent |
|----------|-----------------------------|
| F525_160 |    Single-sided 5.25" 160KB |
| F525_180 |    Single-sided 5.25" 180KB |
| F525_320 |    Double-sided 5.25" 320KB |
| F525_360 |    Double-sided 5.25" 360KB |
| F525_12M | Double-sided HD 5.25" 1.2MB |
| F35_720  |     Double-sided 3.5" 720KB |
| F35_144  | Double-sided HD 3.5" 1.44MB |
| F35_288  | Double-sided XD 3.5" 2.88MB |

You set the ```format``` field to any of the above values as well. This
may yield nonsensical results and DOSContainer will prevent you from doing
something physically impossible like trying to format a 5.25" floppy as if it
could hold 2.88MB of storage or another way around. Particularly within the
same physical form factor you may be doing things that don't make a lot of
practical sense: sure you can format a 1.44MB disk at 720KB of capacity, and
DOSContainer won't stop you from doing that. What you'll get, is a disk image
file that will only ever fill up to half capacity.. but hey, knock yourself
out!

### Floppy file system support

Originally, IBM PC-DOS and MS-DOS only supported FAT12 on any kind of disk.
DOSContainer allows you to format floppies with any filesystem you choose in
the name of end-use flexibility. That's why the ```filesystem``` field is
here: your collection, your choice. It is, however, completely optional and
DOSContainer is smart enough to determine what you need from the combination
of the physical disk and the specific operating system you defined. Yes, it
takes into account the subtle differences and tweaks the specific versions of
the operating systems implemented but only if you give it a combination that
makes sense in reality.

| Value  | File system version                               |
|--------|---------------------------------------------------|
|  FAT12 | FAT12 als implemented by any specific DOS version |
|  FAT16 | The original FAT16 from MS-DOS                    |
| FAT16B | Tweaked version of FAT16 from MS-DOS              |
|   VFAT | FAT16 with long file name support from Windows 95 |
|  FAT32 | FAT32 as implemented in Windows 95OSR2/98         |

Using things like FAT32 on a tiny floppy does nothing for your system except
waste a lot of space and performance. When in doubt, don't specify anything
for the file system and let DOSContainer figure it out for you. It'll do a
realistic job for anything you throw at it, including hard disks of any size
supported later on.

## Foundation layer

The Foundation layer is the second layer that is mandatory on every Manifest
in DOSContainer. It defines the operating system that the disk will be loaded
with, and that the computer will use to start up from. The table below shows
the permitted values and their meaning. DOSContainer has quite a bit of intel-
ligence under the hood that makes choices based on the type of disk and the
exact operating system you choose here.

| Value    | Operating system version |
|----------|--------------------------|
| PCDOS100 | IBM PC-DOS 1.00          |
| PCDOS110 | IBM PC-DOS 1.10          |
| PCDOS200 | IBM PC-DOS 2.00          |

This table looks very sparse at the time of this writing. Adding later ver-
sions and operating systems from other vendors is ongoing. Since this is
more than just a matter of gathering up system files, the earlier versions of
DOS take an inordinate amount of work to implement correctly. The table below
shows the planned versions with their values that can be used in a v1 Manifest
file when DOSContainer hits its 1.00 release target. The table below is
subject to change without notice. The versions marked with a * are currently
implemented.

| Value     | Operating system version |
|-----------|--------------------------|
| PCDOS100* | IBM PC-DOS 1.00          |
| PCDOS110* | IBM PC-DOS 1.10          |
| PCDOS200* | IBM PC-DOS 2.00          |
| PCDOS210  | IBM PC0DOS 2.10          |
| PCDOS300  | IBM PC-DOS 3.00          |
| PCDOS310  | IBM PC-DOS 3.10          |
| PCDOS320  | IBM PC-DOS 3.20          |
| PCDOS330  | IBM PC-DOS 3.30          |
| PCDOS400  | IBM PC-DOS 4.00          |
| PCDOS401  | IBM PC-DOS 4.01          |
| PCDOS500  | IBM PC-DOS 5.00          |
| PCDOS502  | IBM PC-DOS 5.02          |
| PCDOS610  | IBM PC-DOS 6.10          |
| PCDOS630  | IBM PC-DOS 6.30          |
| PCDOS700  | IBM PC-DOS 7.00          |
| PCDOS2K   | IBM PC-DOS 2000          |
| MSDOS401  | MS-DOS 4.01              |
| MSDOS500  | MS-DOS 5.00              |
| MSDOS600  | MS-DOS 6.00              |
| MSDOS610  | MS-DOS 6.10              |
| MSDOS620  | MS-DOS 6.20              |
| MSDOS621  | MS-DOS 6.21              |
| MSDOS622  | MS-DOS 6.22              |
| MSDOS700  | MS-DOS 7.00 (Win95)      |

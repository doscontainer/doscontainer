# Welcome to DOSContainer

DOSContainer aims to provide a command line generator for 100% vintage-compatible
floppy and hard disk images for IBM-PC and compatibles. This aims to support my
retro computing hobby and the collection building that goes with it.

> :warning: **I'm importing and cleaning up an older version of my codebase. Broken builds abound!**

## What's the point?

The DOS computing environment consisted of huge numbers of hardware configurations
with all kinds of drivers and compatibility quirks. That fact, coupled with the fact
that there were thousands of games and applications, accounts for literally a bazillion
of possible permutations between all of those moving parts.

In order to help collection builders, DOSContainer aims to facilitate the quick creation
of pristine disk image files that make up a collection. You tweak the manifest file that
serves as input, then DOSContainer pops out a cleanly generated new disk image that is
configured precisely to your specifications:

- File format suitable for use in emulators or for copying straight to old hardware.
- 100% correct boot sector code
- FAT filesystem quirks correspond with the OS version.
- Install bare minimal base OS
- Middleware, drivers, memory management etc.
- Your game or application itself
- Autoboot configuration in AUTOEXEC.BAT

## Use case?
You want all of your hundreds of games to boot with EGA configured and Roland MT-32 sound
for the old retro setup you have in the attic? DOSContainer makes it easy to do that. It 
also makes it easy to do the exact same thing but for VGA and SoundBlaster Pro audio for 
the MiSTer in your living room. Did the world move on and deliver a new mouse driver? Swap 
it in across thousands of image files without breaking a sweat.

## Current status

At the time of this writing, I'm consolidating my previous misguided attempt at organization
into this cargo workspace-based repository. Misguided? Indeed: I had all my crates in their
own repositories and hosted my own Cargo registry to integrate the whole lot into builds. As
it turns out, workspaces are a lot easier in a case like this.

So that's where I am now: shoveling code over from my now private repos into this one,
adjusting code as I go because I keep learning as I go. So for all intents and purposes, do
treat this code base as **BADLY BROKEN** and very much incomplete for now. I'll make a release
once the main branch here is capable of doing something useful with PC-DOS 1.00 again. That'll
also be the point where I start a workflow with tickets and feature branches to invite 
outside contribution.

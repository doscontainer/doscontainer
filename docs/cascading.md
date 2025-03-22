# Manifest files and cascading settings

DOSContainer aims to make life easier for collection builders. This means that
a lot of the functionality centers on mass-configurability of game collections. For
this reason, we use a cascading model for configuring your collections following
a few basic rules.

When you port a game for use with DOSContainer, you specify as little as possible in
the game's own manifest file. The base assumption is that every DOS game can be made
to work with *every* setting that DOSContainer allows. While that is obviously not
true at all, it is the working assumption for our configuration hierarchy.

You can set your configuration in a few places:

  - ~/.config/doscontainer/config.toml
  - A config.toml file in your collection's top-level directory
  - A config.toml inside any subdir in your collection
  - The individual game manifest .toml file

The individual game's settings take precedence over anything higher up the chain,
so you may define your operating system as MS-DOS 5.00 at the very top but overrule
it anywhere in your collection so that specific games that *must* have MS-DOS 6.00
(hypothetical case) can have it without breaking the rest of your collection.

The same goes for hardware support. You want VGA everywhere? Go right ahead, but
do put games that don't support VGA into a folder of their own then and set the
config.toml in there to EGA or whatever alternative they do support.

## Example setup

The config.toml in your home directory could look something like this:

```
  [hardware]
  video = VGA
  audio = SoundBlaster15
  mouse = serial
  joystick = false

  [SoundBlaster15]
  address = 220
  irq = 5
  dma = 1
```

DOSContainer tries its best to configure each and every game with VGA graphics,
a serial mouse, no joystick, and a SoundBlaster 1.5 card that lives at address
220h, IRQ 5 and DMA 1.

You can add blocks for all types of hardware your collection should support and
define sensible defaults. Now let's say you're setting up an ancient game that
only supports EGA and no sound hardware. That game would get a different hardware
block:

```
  [hardware]
  video = EGA
  audio = PCSpeaker
```

DOSContainer's final opinion on the hardware specs for your ancient game would
result in this:

```
  [hardware]
  video = EGA
  audio = PCSpeaker
  mouse = serial
  joystick = false
```

This means you set the things you want once, and only override them as needed when
they don't make sense for an individual game or a group of games.

# The Manifest configuration format

DOSContainer uses the TOML file format for the manifest configuration file. This
turned out to be the most human-readable format that has wide-spread support in
the wider community and first-class support in the Rust ecosystem.

This document describes what a Manifest needs to contain. Refer to the cascading.md
document to learn how the different configuration files get merged together to
create complete DOSContainer collections.

It is extremely unlikely that you will ever write all of the file format's settings
out into a single game's manifest file! For that reason, this document describes
the sections that you can use in your collections.

The format and possible values are prone to change as DOSContainer progresses.

## Game metadata

Use the metadata section for fields that can be used for collection management.
The section itself is entirely optional and DOSContainer does not use it itself.

```
  [metadata]
  title = Leisure Suit Larry in the Land of the Lounge Lizards
  year = 1987
  publisher = Sierra On-Line Inc.
  comment = Original version of the first episode in the Larry series.
```

## Manifest metadata

```
  [manifest]
  version = 1
```

Currently we only support the version field to indicate the version of the file
format. Be prepared for epic breakage until the first general release. The version
number won't increment before that point in time.

## Hardware

The hardware section dictates how you want your games to be configured. This is
generally set-up at the upper levels of your configuration hierarchy so that whole
groups of games get tweaked just right for your machine at build time. You can
use individual fields in this section to hard-lock any settings in this section
to a specific value if needed.

Note that if a field has no effect at all on a game, you don't need to override
it. This goes for games that, for instance, don't support any specific sound cards
at all. You can set sound hardware all you want, but since DOSContainer doesn't
have any way to pass that information on to the game it'll simply get ignored and
your game will just use the plain old beeper.

```
  [hardware]
  video = VGA
  music = AdLib
  sfx = SoundBlaster15
  mouse = CuteMouse
  joystick = false
```

### Video field

Determines the graphics hardware to use with your games.

| Value | Hardware configured in the game |
|-------|---------------------------------|
| Hercules | Hercules monochrome graphics |
| MDA  | IBM MDA Monochrome Display Adapter |
| CGA4 | IBM CGA in 320x200 4 colour mode |
| CGA2 | IBM CGA in 640x200 2 colour mode |
| EGA  | IBM EGA and compatibles |
| MCGA | IBM MCGA in 320x200 256 colour mode |
| VGA  | IBM VGA in 320x200 256 colour mode |

### Music and SFX fields

Audio in MS-DOS is a complicated beast. Many games support the use of two separate
sound devices: one for music, one for sampled effects and speech. DOSContainer
reflects this situation by giving you two fields for sound hardware. When a game
supports only one audio device, the ```music``` device takes priority. If you leave
```sfx``` undefined, DOSContainer is smart enough to figure out if the ```music```
device is capable of functioning for sound effects as well. If it is, the game will
use your single device for both purposes. If it's not, there won't be any sampled
effects or speech configured in your game.

| Value | Music | SFX | Hardware configured in the game |
|-------|-------|-----|---------------------------------|
| PCSpeaker | X | X | Standard IBM-PC internal speaker |
| AdLib | X | - |Original 11-voice AdLib synthesizer and compatibles. |
| SoundBlaster10 | X | X | Creative Labs SoundBlaster 1.0 |
| SoundBlaster15 | X | X | Creative Labs SoundBlaster 1.5 |
| SoundBlasterPro10 | X | X | Creative Labs SoundBlaster Pro 1.0 |
| SoundBlasterPro20 | X | X | Creative Labs SoundBlaster Pro 2.0 |
| GameBlaster | X | - | CMS GameBlaster |
| MT32 | X | - | Roland MT-32 |
| GUS | X | X | Gravis Ultrasound (original) |
| GUSMax | X | X | Gravis Ultrasound Max |
| Covox | - | X | Sample player attached to the printer port. |
| MPU401 | X | - | Roland MPU-401 or compatible MIDI interface. |

For each device above you can add a section that configures it more specifically.

```
  [SoundBlaster10]
  address = 220
  irq = 5
  dma = 1
````

You'll recognize these values from the configuration setups for many games. They
tell DOSContainer how your hardware is configured so that the game can take this
into account.

### Mouse field

The ```mouse``` field determines which mouse driver is injected into your game
disk. Set it to ```false``` if you don't want a mouse.

| Value | Mouse driver family |
|-------|---------------------|
| CuteMouse | Loads the CuteMouse driver. |
| Microsoft | Loads the Microsoft mouse driver. |
| Logitech | Loads the Logitech mouse driver. |
| Genius | Loads the Genius mouse driver. |

### Joystick field

The joystick doesn't require any drivers. You can set the field to ```true``` or
```false```. The games will take care of using it.

## Disk section

The disk section is heavily under construction. We only support floppies for now.

```
  [disk]
  type = F35_720
```

For now we only define a type field that supports the following values:

| Value | Floppy type |
|-------|-------------|
| F525_160 | Single-sided 160KB |
| F525_180 | Single-sided 180KB |
| F525_320 | Double-sided 320KB |
| F525_360 | Double-sided 360KB |
| F525_1200 | Double-sided 1.2MB |
| F35_720 | Double-sided 720KB |
| F35_1440 | Double-sided 1.44MB |
| F35_2880 | Double-sided 2.88MB |

More recent DOS versions support all of them, ancient versions are much more
picky so do your homework!

Hard disk support is still in the works.

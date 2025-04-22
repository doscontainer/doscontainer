# Hardware specification format

The HWSpec crate feeds DOSContainer with hardware specifications of your target
machine. It takes TOML file that defines your PC, using the following sections.

## Audio hardware

One of the first additions that became widely available and supported on IBM-
compatible PC's was sound hardware. DOSContainer supports configuring the
following types of hardware:

| Config value | Hardware make/model                 |
|--------------|-------------------------------------|
| AdLib        | AdLib Music Synthesizer Card (1987) |
| CMS          | Creative Music System / GameBlaster |
| SB10         | Creative Labs Sound Blaster 1.0     |
| SB15         | Creative Labs Sound Blaster 1.5     |
| SB20         | Creative Labs Sound Blaster 2.0     |
| SBPRO        | Creative Labs Sound Blaster Pro     |
| SBPRO2       | Creative Labs Sound Blaster Pro 2   |
| SB16         | Creative Labs Sound Blaster 16      |
| SBAWE32      | Creative Labs Sound Blaster AWE32   |
| MT32         | Roland MT-32                        |
| LAPC1        | Roland LAPC-1                       |
| MPU401       | Roland MPU-401 General Midi compat. |
| SC55         | Roland SC-55                        |
| SCC1         | Roland SCC-1                        |
| COVOX        | Covox SpeechThing & compatibles     |
| GUS          | Gravis UltraSound                   |
| GUSMAX       | Gravis UltraSound MAX               |

It used to be fairly common to have more than one piece of audio hardware
in a single PC. You can specify any number of sound cards in your hwspec file,
but it's up to the system to actually work. DOSContainer doesn't stop you:

```
[[ audio ]]
device = SB15
io = "220"
dma = "1"
irq = "5"

[[ audio ]]
device = MT32
io = "330"
```

The above example defines a common combination: a Creative Labs Sound Blaster
1.5 for sampled audio output (sound effects, speech) and a Roland MT-32 for
music.

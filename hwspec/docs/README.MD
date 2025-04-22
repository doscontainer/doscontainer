# Hardware specification format

The HWSpec crate feeds DOSContainer with hardware specifications of your target
machine. It takes TOML file that defines your PC, using the following sections.

## Setting the CPU

You can set the CPU for your system as a base property:

```
[base]
cpu = "8088"
```

While CPU types are historically often bare numbers, these values are not
numerical in the configuration file format. You must quote them! Here's the
table of supported values and the actual CPU chip the represent:

| Config value | CPU chip model |
|--------------|----------------|
| 8086         | Intel 8086     |
| 8088         | Intel 8088     |
| 80186        | Intel 80186    |
| 80286        | Intel 80286    |
| 80386SX      | Intel 80386SX  |
| 80386DX      | Intel 80386DX  |
| 80486SX      | Intel 80486SX  |
| 80486SX2     | Intel 80486SX2 |
| 80486SL      | Intel 80486SL  |
| 80486DX      | Intel 80486DX  |
| 80486DX2     | Intel 80486DX2 |
| 80486DX4     | Intel 80486DX4 |

For now, we only support specifying Intel CPU's. If ever a software title
comes along that *requires* that we specify something non-Intel, it'll be
added to the list so that it can be properly configured.

## Audio hardware

One of the first additions that became widely available and supported on IBM-
compatible PC's was sound hardware. DOSContainer supports configuring the
following types of hardware:

| Config value | Hardware make/model                 |
|--------------|-------------------------------------|
| Bleeper      | Standard IBM-PC internal speaker    |
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

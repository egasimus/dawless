(Source: https://gist.github.com/jack126guy/b2d38db0c96ca247ae1ad385e011fd78#file-electribe-sampler-e2ssample-format-md)

# Electribe Sampler e2sSample.all Data Format

Based on examination of the source code for [e2sEdit](http://flosaic.com/e2sEdit/) and [Oe2sSLE](https://github.com/JonathanTaquet/Oe2sSLE).

## Overview

The file is just a concatenation of WAVE files with a header to indicate where each file begins. Additional metadata such as sample name and loop point are stored in each WAVE file as a RIFF chunk with type "korg", which is conventionally at the end of the file.

Unless otherwise specified, all integers are little-endian unsigned.

## Header

The header is 4096 bytes long.

Offset 0 (0x0000): Header, "e2s sample all" in ASCII, followed by nulls.

Offset 88 (0x0058): Sample offsets. Each offset is a 32-bit integer. If a sample is not present, the offset is set to 0.

The remainder of the header consists of nulls.

## Metadata RIFF Chunk

The chunk consists of a single subchunk with type "esli" and data length 1172. Offsets are with respect to the subchunk.

Offset 0 (0x0000): Sample number, except zero-based so it would be one less than the number displayed, as a 16-bit integer.

Offset 2 (0x0002): Sample name in ASCII, padded by nulls to 16 bytes.

Offset 18 (0x0012): Category number as a 16-bit integer. Starting at 0, the categories are: Analog, Audio In, Kick, Snare, Clap, HiHat, Cymbal, Hits, Shots, Voice, SE, FX, Tom, Perc., Phrase, Loop, PCM, User.

Offset 20 (0x0014): "Absolute sample number" (e2sEdit) or "import number" (Oe1sSLE) as a 16-bit integer. For the factory samples, it seems to start at 50 and increment for each channel (i.e., one for mono samples and two for stereo samples). Not entirely sure what this is used for.

Offset 22 (0x0016): Fixed string: 0x00 0x00 0x00 0x7F 0x00 0x01 0x00 0x00 0x00 0x00 0x00 0x00

Offset 34 (0x0022): "Playback period," according to Oe2sSLE, as a 16-bit integer, which by default is calculated as 63132 - log2(sample rate) * 3072 rounded to the nearest integer.

Offset 36 (0x0024): Playback volume as a 16-bit integer.

Offset 38 (0x0026): Two nulls (although Oe2sSLE seems to acknowledge that the first byte may vary).

Offset 40 (0x0028): Start point as a 32-bit integer offset into the audio data.

Offset 44 (0x002C): Loop point as a 32-bit integer offset into the audio data.

Offset 48 (0x0030): End point as a 32-bit integer offset into the audio data. For both the loop and end point, it seems that the "end" of the sample is two less than the data length.

Offset 52 (0x0034): Loop flag as a 8-bit integer: 0 = loop, 1 = one-shot.

Offset 54 (0x0035): Seven nulls.

Offset 60 (0x003C): Data length of the WAVE file.

Offset 64 (0x0040): The byte 0x01.

Offset 65 (0x0041): Stereo flag as an 8-bit integer: 0 = mono, 1 = stereo.

Offset 66 (0x0042): Loudness ("play level") flag as an 8-bit integer: 0 = normal, 1 = +12dB.

Offset 67 (0x0043): Fixed string: 0x01 0xB0 0x04 0x00 0x00.

Offset 72 (0x0048): Sample rate as a 32-bit integer.

Offset 76 (0x004C): A null.

Offset 77 (0x004C): Sample tune as an 8-bit signed two's-complement integer.

Offset 78 (0x004E): Zero-based sample number again.

The remainder of the chunk consists of nulls, at least according to e2sEdit. According to Oe2sSLE it is the slice data.

# Rcolor

This Rust command-line application processes an input image by replacing each pixel color with the closest color from a provided list of colors. The list of colors is specified in a text file with hex color values.

## Usage

1. **Prepare Color File:**
   Create a text file containing hex color values (without the `#` symbol) for each line. Save this file with a ".hex" extension, for example, "colors.hex". The hex file should contain one color per line.

   Example "colors.hex":

```
172038
253a5e
3c5e8b
```



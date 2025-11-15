# Makepad Shadecn Core Resources

This directory contains fonts and other resources for the makepad-shadecn library.

## Required Fonts

Please place the following fonts in this directory:

1. **IBMPlexSans-Regular.ttf** - Used for:
   - Input fields (ShadecnInput)
   - Checkboxes (ShadecnCheckbox)
   - Card descriptions (ShadecnCardDescription)
   - Labels and general text throughout the demo

2. **IBMPlexSans-SemiBold.ttf** - Used for:
   - Buttons (ShadecnButton)
   - Card titles (ShadecnCardTitle)
   - Headings in the demo

## Where to get these fonts

These fonts are part of IBM Plex, an open-source typeface family. You can download them from:
- https://github.com/IBM/plex/releases
- Or copy them from `widgets/resources/` in the main makepad repository

## Font Files

Once you place the fonts here, they will be referenced in the code as:
- `dep("crate://makepad-shadecn-core/resources/IBMPlexSans-Regular.ttf")`
- `dep("crate://makepad-shadecn-core/resources/IBMPlexSans-SemiBold.ttf")`


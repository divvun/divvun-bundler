# Divvun Bundler

Create installer packages for all Divvun spell checking utilities

## Usage

- Download and install NSIS from https://nsis.sourceforge.io/Main_Page
- Install the LockedList plugin https://nsis.sourceforge.io/LockedList_plug-in into the Plugins folder of the NSIS installation directory (extract the Plugins folder from the LockedList archive into C:\Program Files (x86)\NSIS\Plguins)

## On Windows:
Run the divvun-bundler binary from a Windows SDK (or Visual Studio) command line prompt, as it depends on signtool

### On Linux/Mac:
- Have wine installed
- Download precompiled osslsigncode binary from https://sourceforge.net/projects/unix-utils/files/osslsigncode/

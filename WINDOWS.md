# Using 32-bit pvoc plugins on Windows with Ladspashell VST
*The following instructions are written by Simon W. Thanks!*

This guide is on how to use pvoc-plugins (and other LADSPA plugins
compiled for Windows) with non-LADSPA supporting hosts (most of the DAW’s for
Windows). 

If you’re planning on using pvoc-plugins with a host that
does support LADSPA plugins natively (eg. Audacity), you don’t have to use the
information from this guide, since your host natively supports it. The first
step is crucial on Windows though (for pvoc-plugins!).

Download link: https://drive.google.com/drive/folders/1-IK3T-QXMOCwLJkK1MKgSMGxKYek-6Xs

1. Download the compiled plugins for Windows and extract the archive.  2. Copy
"**libgcc\_s\_dw2-1.dll**" to "**C:\Windows\system**", which is one of MinGW's
library files, that's required for the plugins to run, because they were
compiled for Windows using MinGW.  3. Place the "**pvoc\_plugins.dll**" in some
directory of your choosing, for example "C:\Program Files (x86)\LADSPA
Plugins\pvoc\pvoc\_plugins.dll"

- It's important that the plugins are put in a one directory lower (the pvoc
  folder in the example), because otherwise "**ladspashell**" won't detect
  them! So a directory like that: "C:\Program Files (x86)\pvoc\here are the
  plugins" will result in the plugins not being detected!

4. If your host supports shell plugins (most nowadays do), place
"**ladspashell**.**dll**" in your VST directory.

- On the next scan your host will detect it and then it will ask you to choose
  the directory for LADSPA plugins.

- Based on the example directory above choose something like "C:\Program Files
  (x86)\LADSPA Plugins", inside of which there is a folder "pvoc" containing
  the pvoc-plugins, and not a directory "directly" containing the plugins, that
  it one that's has the pvoc-plugins library by themselves, and not inside of a
  folder a directory lower, of the directory chosen inside of the
  "ladspashell"'s prompt.

5. If everything went right, you should now be able to use "**pvoc-plugins**"
on Windows, in a DAW that doesn't natively support LADSPA plugins using a
LADSPA VST wrapper!

### If your host doesn't support shell plugins, then use the packaged **shell2vst** program inside the "**shell2vst**" folder.
Here are the steps: 1.
Drag the "**ladspashell**.**dll**" onto the "**shell2vst**.**exe**" and then
you'll be prompted to choose the directory for LADSPA plugins. Then it will
generate a folder called "**ladspa**", inside of which there will be VST
versions of pvoc plugins.  2. Copy that folder into your VST directory.

- If you have chosen the directory of LADSPA plugins before, **shell2vst** will
  just generate VST's from the LADSPA's of your chosen directory.


### Important notice:
If you encounter some error during the **ladspashell**
scan after choosing the directory of LADSPA plugins, you'll have to delete the
registry file of the loader, because otherwise "**ladspashell**" won't scan for
new plugins (because it had already scanned for the non-working pvoc plugins),
so in that case just delete the registry key
"**HKEY\_CURRENT\_USER\Software\Polac\" and the folder **Polac** in
AppData\Roaming, and on the next scan of your host "**ladspashell**" will once
again prompt you to choose the directory of LADSPA plugins.

Same goes for wanting to add more LADSPA plugins compiled for Windows and have
them scanned by "**ladspashell**"

Credit for LADSPA shell VST and shell2vst: [Polac](https://www.xlutop.com/buzz/)

# Cross compiling for Windows
If the binary linked above does not work for you or you would prefer to compile, here are instructions for doing so.
Currently compilation on Windows directly is not tested, and the below directions are for cross compiling from Linux.
There are some issues when compiling with the latest stable Rust. Here's how to set up the known working version:

```
rustup default nightly-2020-06-27
rustup target add i686-pc-windows-gnu
cargo build --release --target i686-pc-windows-gnu
```

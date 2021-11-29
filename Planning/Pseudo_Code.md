

# Pseudocode

The order of which what happens

**Main**

1.  get arguments from launch
2.  figure out if there is a configuration file
    *   if there isn't a file then create one
        1.  <span class="todo">TODO</span>: Figure out how to detect package managers
        2.  Set default from a list (certain compiler flags will allow Snap or Flatpak to be main)
        3.  Set possible package managers (certain ones can be disabled at compile time)
    *   else get default manager from config file
3.  check for option flag
    *   if particular pkmgmg is selected then execute that package manager (`-flatpak`)
    *   if `-init` then rewrite configuration file
    *   else execute particular package manager function block
4.  return 0

**Package Manager (Using Pacman as example)**

1.  check for command (install, remove, update, etc)
    *   if there is a command then execute the proper command
        *   if the command requires more args, then pass the remaining arguments into the command
            *   Ex: if `install` then execute `pacman -S (package name)` and pass all remaining arguments as arguments
                *   should also work with local packages `pacman -U /path.rpm`
    *   if there is no command (install, remove, update, etc)
        *   then pass all remaing arguments into `pacman`
2.  capture all output from `pacman`
    *   parse output and possible add colors
    *   make warnings or errors very very obvious
3.  return

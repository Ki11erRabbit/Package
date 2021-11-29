# Package
A universal package manager wrapper




## Goals

*   create a universal interface for all package managers
*   be barely slower than base package manager
*   be modular so that newer package managers can easily be added to the code



### Side Goals

*   be powerful enough to be given a GUI as a higher layer of abstraction


## Useful Links

* [Executing Programs with Sudo](https://stackoverflow.com/questions/48791210/c-programming-execute-another-program-with-sudo-privileges) 
* [Executing Linux Commands](https://stackoverflow.com/questions/4757512/execute-a-linux-command-in-the-c-program)



## How the Program Operates

1.  At first runtime, figure out what package managers (Apt, Pacman, Flatpak, etc) are installed
2.  Then write a configuration file that stores what managers are installed
3.  This then can be redone by doing `package -init` to refresh the config file
4.  The config file will then tell what packages manager to run as the default
5.  Certain comands (i.e. `-flatpak`, `-aur`, `-apt`, `-snap`, etc) will bypass the default and execute the associated package manager (flatpak, the AUR, Apt, Snap, etc)

*   Have the primary packages managers (Pacman, Portage, Apt, Zypper, etc) be set as the the possible defaults
*   Have condition flags that can force Snap or Flatpak (or others) as the default instead (for Ubuntu, Elementary OS, & Fedora)

## Important Things
*   Behavior must be consistant (i.e. portage will always be set to ask unless otherwise)
*   all warnings from the original package manager should be made obvious to the user (no Steam uninstalling the desktop environment)



### Inspirations

</div>

*   Linus from Linus Tech Tips Linux Challenge
    *   [Part 1 Apt uninstalling the DE](https://www.youtube.com/watch?v=0506yDSgU7M)
        *   Apt's warning about system packages gets hidden in verbose language
        *   Apt uninstalled the Desktop Environment when installing a program which should not be possible
    *   [Part 2 Apt-get-ting in Manjaro](https://www.youtube.com/watch?v=3E8IGy6I9Wo)
        *   There are tons of guides on the internet that assumes that the user is using a Debian/Ubuntu based distros, causing confusion
*   Smoothing over the fractured nature of Linux
    *   Creating software that can easily be included in "Just Works" Distros for beginners
*   Something that I can use to learn Software Engineering
    *   Learn how to make a powerful comandline program
    *   Learn how to make a program execute other programs and execute them with root privileges when given root privileges


# Package's Commands





## Most Important Commands (for novices)


Should call the coresponding

*   Install (Should work for local packages)
*   Remove
*   Upgrade/Update
*   Search
*   Set (takes a package manager name to set as the default)


## Secondary Importance Commands (for normal users)


*   Clean
*   Force (ignores any complaints. aka dangerous)


### Options For Program


*   `-` then `pkmgr name` for selecting a particular package manager to do the specified actions
    *   also you can pass normal arguments relative to each package manager (i.e. `package -pacman -Syy`)
*   `-init` rewrites the configuration file and does whatever action action relative to the config file if there are other commands



### Format of usage

`package (Options) (Command) (Package names)`



#### Examples

</div>

*   `package install gimp`
*   `package -flatpak install gimp`
*   `package -apt install gimp`
*   `package -init install gimp`
*   `package install ~/Downloads/gimp-3.1.4.deb`
*   `package Update`
*   `package set -snap`



##### Helpful Links



* [Pacman Rosetta](https://wiki.archlinux.org/title/Pacman/Rosetta)
* [Opensuse Software Managerment Comparison](https://old-en.opensuse.org/Software_Management_Command_Line_Comparison)

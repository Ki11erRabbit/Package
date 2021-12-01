
# Package's Commands





## Most Important Commands (for novices)


Should call the coresponding

*   Install (Should work for local packages)
*   Remove
*   Upgrade/Update
*   Search
*   Set (takes a package manager name to set as the default)
*   Pass (passes arguments directly to default package manager
*   Initialize (rewrites the config file)


## Secondary Importance Commands (for normal users)


*   Clean
*   Force (ignores any complaints. aka dangerous)


### Options For Program (WIP)

*   Use followed by a hyphen and the package manager of choice (`package install use-apt` or `package pass use-apt`)




### Format of usage

`package (Command) (Options) (Package names/other args)`



#### Examples

</div>

*   `package install gimp`
*   `package install -use-flatpak gimp`   
*   `package install ~/Downloads/gimp-3.1.4.deb`
*   `package pass -Syu`
*   `package pass -use-apt install gimp`
*   `package initialize`
*   `package update`
*   `package set snap`



##### Helpful Links



* [Pacman Rosetta](https://wiki.archlinux.org/title/Pacman/Rosetta)
* [Opensuse Software Managerment Comparison](https://old-en.opensuse.org/Software_Management_Command_Line_Comparison)

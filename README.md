# Rust Asset Manager
Rust Asset Manager is a CLI tool for importing assets to your project. It does this by copying and pasting files from the Downloads folder (or any other folder you set) to the assets directory in your project.

### Why?
As I started using the terminal and Neovim more and more, I felt there was too much friction when importing pictures to my projects. Using the mv command was slow because I had to type the entire file path and the relative path to the assets folder.

Another solution was using Finder to locate and import a file. Leaving the terminal and navigating through files seemed even slower than using the mv command.

## Features
Rust Asset Manager has following features:
- ls
- add
- remove
- default
- set-default
- import


### ls
`assetm ls`
will list all projects that have been added to assetm.

### add
`assetm add <project_name>` will add the current directory to the list of projects. To use this command properly, you should navigate to your projects assets directory, for example,  `~/Desktop/{some_project}/src/assets`. After that, you can use the **add** command to add it.

If we used the `assetm add <project_name>` after navigating to the route mentioned above., this command would add the name {project_name} and ``~/Desktop/{some_project}/src/assets`` as the path.

### remove
`assetm remove <project_name>` removes the project from the list of projects.

### default
`àssetm default` prints out the default path from which to pull the assets. The initial value for this is the Downloads folder.

### set-default 
`assetm set-default` will take the current directory and set it as the default path from which to pull the assets. This is useful, for example, if you download assets to the Desktop instead of the Downloads folder. In that case, you would navigate to the Desktop and use the set-default command.
### import
`assetm import <project_name> <file_to_import>` will copy the file from the default path to the project's directory.

**Example:** `assetm import some-project picture1.png`<br>

This command will pull `picture1.png` from `~/Downloads/picture1.png` (if the default wasn't changed) and  import it to the path associated with the project.


---

I use vim btw<br>
Written in Rust btw.



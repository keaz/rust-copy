# A Replacement for cp

This is a cli tool that try to replace unix cp command. This provides better performance than cp and more suitable for SSD.


## Usage
### Install Using cargo
```
cargo install --path .
```
### Execute
```sh
rfcp -s=/source/folder -d=/destination/folder
```

### Install Using Homebrew
```
brew install keaz/homebrew/rfcp
```

## Examples

### Copy root folder with content the options.
```sh
rfcp -s=/source/folder -d=/destination/folder
```

### Copy content without creating the root folder.
```sh
rfcp -s=/source/folder/ -d=/destination/folder
```

## Options
-s : Souce folder
-d : Destintion folder
-t : Number of threads to use copy files  (default 3)
-r : Number of threads to use read the filed (default 1) Note: for HDD use the default
-b : Buffer size (default 10240)

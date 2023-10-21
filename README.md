# A Replacement for cp

This is a cli tool that try to replace unix cp command. This provides better performance than cp and more suitable for SSD.


## Usage
### Install
```
cargo install --path .
```
### Execute
```sh
rfcp -s=/source/folder -d=/destination/folder
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
-t : Number of threads (default 3)
-b : Buffer size (default 10240)

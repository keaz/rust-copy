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

### With all the options.
```sh
rfcp -s=/source/folder -d=/destination/folder -t=10 -b=10240
```
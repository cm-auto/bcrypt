# bcrypt

Simple command line application that allows hashing and verifying passwords.\
Useful if you want to insert dummy data into your development database.

# Usage

This program has two sub commands: `hash` and `verify`, if none is specified the
former is assumed.

## hash

```console
Usage: bcrypt hash [OPTIONS] <PASSWORD>

Arguments:
  <PASSWORD>  

Options:
  -c, --cost <COST>  [default: 12]
  -h, --help         Print help

Error Codes:
  Success:              0
  Verification Failed:  1
  Error:               10
```

## verify

```console
Usage: bcrypt verify <PASSWORD> <HASH>

Arguments:
  <PASSWORD>  
  <HASH>      

Options:
  -h, --help  Print help

Error Codes:
  Success:              0
  Verification Failed:  1
  Error:               10
```

Don't forget to escape the `$` characters of the hash, when using this program
in a shell!\
Example in bash:

```console
bcrypt verify foo \$2b\$12\$Pre8i2FmGdJ7tO0bX34aEug3z8fx.7.DObvl2PfvI9BXKxAynwT7.
```

<!--
 * @Author: FunctionSir
 * @License: AGPLv3
 * @Date: 2024-12-21 21:33:14
 * @LastEditTime: 2024-12-22 00:32:14
 * @LastEditors: FunctionSir
 * @Description: -
 * @FilePath: /anyflector/README.md
-->
# anyflector

Reflector-like tool for mirror lists.

## To build

Just use "cargo":

``` bash
    cargo build --release
```

The compiled binary is target/release/anyflector

## To use

``` bash
    anyflector /any/mirror/list repo-name max-time-for-curl
```

Do NOT change the args' positions.

**Important: Please BACKUP your mirrorlists first! This will OVERWRITE the file! And NO comments will be kept!**

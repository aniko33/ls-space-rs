
<h1 align="center">LS-SPACE-CLI</h1>

## TOC

- [Installation](#installation-guide)
- [Usage](#usage)
- [Showcase](#showcase)
- [License](#license)
- [Contributions](contributions)
- [About fork](#about-fork)

## Installation guide

### From SRC
```bash
git clone https://github.com/aniko33/ls-space-rs.git
cd ls-space-rs
cargo build -r 
```
The compiled file is located in this path: `target/release/lsspace`
#### Copy file in bin
`sudo mv target/release/lsspace`


### From binary
[Download binary](https://github.com/aniko33/ls-space-rs/releases)

#### ***go to the folder where you installed the file is run this command***
```bash
sudo mv lsspace-yourplatform /bin
```
Replace *"yourplatform"* with the platform of the file you downloaded for example: *x86_64-linux*

## Usage
```txt
Simple program to scan memory disk  
  
Usage:  lsspace [OPTIONS] --path <PATH>  
  
Options:  
-p, --path <PATH> Path to be scanned  
-c, --chars <CHARS> Number of chars [default: 30]  
-h, --help Print help  
-V, --version Print version
```

### Example commands
Scan root dir: `lsspace -p /`

Scan root dir & change numbers of bar graph: `lsspace -p / -c 100`

## Showcase
![image](https://github.com/aniko33/ls-space-rs/assets/76649588/a060c055-2a62-4da3-bb8d-2bb20d49406c)

![image](https://github.com/aniko33/ls-space-rs/assets/76649588/7a23d140-4f0e-4b29-b469-3c3dabc053c8)

## License
This application is distributed under the _**[GPL](https://it.wikipedia.org/wiki/GNU_General_Public_License) license**_ you can _**consult the file**_: _**[LICENSE](https://github.com/aniko33/Charles-CSEC/blob/main/LICENSE.txt)**_

## Contributions
<a href="https://github.com/aniko33/Charles-CSEC/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=aniko33/ls-space-rs"/>
</a>

## About fork
This fork was done as an update of [ls-space-cli](https://github.com/aniko33/ls-space-cli) done in Python, this version in Rust is mostly a "porting".
**The purpose of this project is only for training and updating a repository, there will be no imported updates**

<a name="readme-top"></a>


## About 

This repo contains 3 independent modules for executing the mappings of an 
RML document using algebraic expressions. 


## Prerequisites

To compile the project on your own, you'll need to have
[Rust toolchain](https://www.rust-lang.org/tools/install) installed.

For Linux-based users:

- Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

For the visualization of the generated mapping plans, you'll need
python version >= 3.10 and the following packages:

1. graphviz


### Python requirements 

For running the normalz

## Running

1. Download this repo
2. Run cargo build at the root this repo
   ```sh
   cd {repo dir}
   cargo build --release
   ```
3. Run the CLI translator app from the compiled translator binary
   ```sh
   cd ./target/release/
   ./eswc_translator  file  <RML_DOCUMENT>
   ```
   For more information/options of CLI app:
   ```sh
   ./eswc_translator  -h
   ```
4. Visualize the created mapping plan
   ```sh
   dot -Tpng {generated dot file} > output.png
   ```
5. Simple plain text format of the mapping plan for parsing
   ```sh
   dot -Tplain {generated dot file} > output.txt
   ```
   <p align="right">(<a href="#readme-top">back to top</a>)</p>


## Acknowledgement

This software makes use of [sophia_rs](https://github.com/pchampin/sophia_rs) crate!

<p align="right">(<a href="#readme-top">back to top</a>)</p>


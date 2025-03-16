<a name="readme-top"></a>

## About

This repo contains 3 independent modules for executing the mappings of an
RML document using algebraic expressions.

1. RML normalization (normalizer.py)
2. RML translation to algebraic expressions (rust-translator)
   - Adapted the `vocab`, `operator`, and `plangenerator` modules from
     [algemaploom-rs](https://github.com/RMLio/algemaploom-rs/)
3. Algebraic expression evaluation engine (rmlweaver-js)
   - Added new operators such as union and extend functions to the prototyping 
   algebraic mapping engine [RMLWeaver-JS](https://github.com/RMLio/rmlweaver-js/)

## Prerequisites

To compile the project on your own, you'll need to have
[Rust toolchain](https://www.rust-lang.org/tools/install) installed.

For Linux-based users:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For the visualization of the generated mapping plans, you'll need
python version >= 3.10 and the following packages:

1. graphviz

### Python requirements

For running the normalizer script, here are the required modules:

```
rdflib==6.1.1
```

## Testing

The test cases reside inside the folder `rmlweaver-js/test` split into two
folders according to the format of the input data, either CSV or JSON.
The test cases are drawn from the website of [https://rml.io/test-cases/](https://rml.io/test-cases/)

Test cases can be run as follows:

1. Navigate to the folder `rmlweaver-js`

2. Run `npm install && npm test`

## Running

### Step 1: Normalizing RML document

1. Run the normalizer python script

```sh
python3 normalizer.py  -f <RML_DOCUMENT>
```

2. The normalized document will be generated with the extension ".normalized.ttl"

3. (Optional) For more options

```sh
python3 normalizer.py -h
```

### Step 2: Translating normalized RML document to mapping plan

1. Run cargo build at the root this repo
   ```sh
   cargo build --release
   ```
2. Run the CLI translator app from the compiled translator binary

   ```sh
   cd ./target/release/
   ./eswc_translator  file  <NORMALIZED_RML_DOCUMENT>
   ```

   For more information/options of CLI app:

   ```sh
   ./eswc_translator  -h
   ```

3. A dot file will be generated named "plan.dot"

4. (Optional) Visualize the created mapping plan
   ```sh
   dot -Tpng {generated dot file} > output.png
   ```
5. (Optional) Simple plain text format of the mapping plan for parsing
   ```sh
   dot -Tplain {generated dot file} > output.txt
   ```

### Step 3: Evaluating the generated mapping plan with an algebraic mapping engine

1. Run npm command to execute the mapping engine for the given mapping plan in
   dot format

```sh
npm run execute_dot <Dot file>
```

## Acknowledgement

This software makes use of [sophia_rs](https://github.com/pchampin/sophia_rs) crate!

<p align="right">(<a href="#readme-top">back to top</a>)</p>

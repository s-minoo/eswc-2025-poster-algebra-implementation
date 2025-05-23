<a name="readme-top"></a>

# RML Compliance of a Formal Algebra for Knowledge Graph Construction
This is a prototype implementation of an RML-compliant mapping engine 
based on the formal algebra from the research paper 
*An Algebraic Foundation for Knowledge Graph Construction* [^paper_fnt]. 

## Background 
Current declarative mapping languages lack the formal foundations for capturing 
the semantics of mapping heterogeneous data to knowledge graphs (i.e., RML for
RDF graph mapping).
The research paper aims to address the lack of formal semantics for declarative
mapping languages by introducing formally defined algebra for capturing 
mapping semantics.
Another contribution from the paper is the translation algorithm from the 
popular mapping language RML to the algebra -- thereby, providing a formal 
definitions for the semantics of RML. 


## Purpose
The focus of this prototype implementation is to empirically verify that the 
translation algorithm provided by the paper[^paper_fnt] aligns with the informally-defined semantics of RML 
by evaluating the prototype RML-compliant mapping engine with official [RML test cases](https://rml.io/test-cases/).

<p align="center">
<img src="./figures/pipeline.svg">
</p>
<p align="center">Figure 1. Three-step execution of an RML-compliant mapping
engine based on formal algebra </p>

The execution flow of this prototype implementation is shown in Figure 1
with the following steps: 

1. A **valid** arbitrary RML v1.1.2 compliant document is first normalized using the 
python script [normalizer.py](./normalizer.py)
2. The normalized RML RDF-graph, from step 1, is translated to algebraic expressions (module: [rust-translator](./rust-translator/))
   - `vocab`, `operator`, and `plangenerator` modules from
     [algemaploom-rs](https://github.com/RMLio/algemaploom-rs/) are adapted for
     the translator implementation.
3. The generated algebraic expression is evaluated, using the execution engine, to map heterogeneous data to RDF graph
   (module: [rmlweaver-js](./rmlweaver-js/))
   - New operators such as union and new extend functions are implemented ontop 
   of the prototyping algebraic mapping engine [RMLWeaver-JS](https://github.com/RMLio/rmlweaver-js/) to 
   evaluate the algebraic expressions generated in step 2.



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

## License 
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.



[^paper_fnt]: Min Oo, S., Hartig, O.: An Algebraic Foundation for Knowledge
    Graph Construction. In: Proceedings of the 22nd Extended Semantic Web
    Conference (ESWC). Springer Nature Switzerland (2025), extended version is 
    available at: [https://arxiv.org/abs/2503.10385](https://arxiv.org/abs/2503.10385)



<p align="right">(<a href="#readme-top">back to top</a>)</p>

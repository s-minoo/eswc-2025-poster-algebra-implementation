#!/usr/bin/env bash

cargo b
../target/debug/eswc_translator -o ./test-resources/ file ./test-resources/sample_mapping.ttl

#!/usr/bin/env python3

"""
File: normalizer.py
Author: Sitt Min Oo
Email: x.sittminoo@ugent.be
Github: https://github.com/s-mioo
Description: Normalize the input RML documents
"""

import argparse
import logging
import os
from types import NoneType

import rdflib
from rdflib.graph import Graph

logger = logging.getLogger(__name__)


def cmdline_args():
    # Make parser object
    p = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )

    p.add_argument("-f, --file", type=str, help="input RML document file")
    p.add_argument("--folder", type=str, help="the folder containing RML documents")
    p.add_argument("-d", "--debug", action="store_true", help="toggle debug mode")

    return p.parse_args()


def class_shortcut_expand(g: Graph):
    logger.debug("Executing class shortcut expand query ")
    g.update(
        """
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 

        DELETE { ?sm rr:class ?sm_class . }
        INSERT {
             ?tm rr:predicateObjectMap [
                 rr:predicateMap [
                     rr:constant rdf:type; 
                     rr:termType rr:IRI 
                 ]; 
                 rr:objectMap  [
                     rr:constant ?sm_class; 
                     rr:termType rr:IRI
                 ]
             ]
        }
        WHERE {
            ?tm rr:subjectMap ?sm . 
            ?sm rr:class ?sm_class . 
        }

             """
    )
    logger.debug("")
    pass


def handle_file(file: str):

    if file.endswith(".ttl"):
        with open(file) as f:
            g = rdflib.Graph().parse(file)
            class_shortcut_expand(g)

            pass
        pass

    pass


def handle_folder(folder: str):
    for root, dirs, files in os.walk(folder):
        for file in files:
            handle_file(os.path.join(root, file))
            pass
        for dir in dirs:
            handle_folder(os.path.join(root, dir))
            pass
        pass
    pass


def main():
    args = cmdline_args()
    log_level = logging.INFO
    if args.debug:
        log_level = logging.DEBUG

    logging.basicConfig(level=log_level)

    if args.file is not None:
        handle_file(args.file)
        pass
    elif args.folder is not None:
        handle_folder(args.folder)
        pass
    pass


if __name__ == "__main__":
    main()

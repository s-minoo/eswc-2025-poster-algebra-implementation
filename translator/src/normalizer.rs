use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use oxigraph::sparql::{QueryOptions, QueryResults};
use oxigraph::store::Store;

use crate::error::oxigraph::OxigraphError;

pub fn normalize_rml(store: &Store) -> Result<&Store, OxigraphError> {
    shortcut_expand_class(store)
        .and_then(shortcut_expand_constant_value)
        .and_then(deduplicate_multi_pom_to_singleton)
        .and_then(replace_self_ref_maps)
        .and_then(duplicate_tm_with_mulitple_poms)
        .and_then(ensure_tm_smgm_or_pomgm)
        .and_then(triples_maps_one_graph_map)
}

fn shortcut_expand_class(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
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
        ",
    )?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("sm_class_constant_expand.ttl")).unwrap();
    }

    Ok(store)
}

fn shortcut_expand_constant_value(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 

        DELETE { 
            ?tm rr:subject ?sm_constant.
            ?pompm rr:predicate ?pm_constant.
            ?pomom rr:object ?om_constant. 
            ?termMap rr:graph ?gm_constant.
        }
        INSERT {
             ?tm rr:subjectMap [ 
                rr:constant ?sm_constant
             ].

             ?pompm rr:predicateMap [
                rr:constant ?pm_constant
             ].

             ?pomom rr:objectMap [
                rr:constant ?om_constant
             ].

             ?termMap rr:graphMap [
                rr:constant ?gm_constant
             ].
        }
        WHERE {
                    { ?tm rr:subject ?sm_constant . } 
            UNION   { ?pompm rr:predicate ?pm_constant . } 
            UNION   { ?pomom rr:object ?om_constant . } 
            UNION   { ?termMap rr:graph ?gm_constant . } 
        }
        ",
    )?;
    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("all_constant_expand.ttl")).unwrap();
    }

    Ok(store)
}

fn deduplicate_multi_pom_to_singleton(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom. 
            ?pom rr:predicateMap ?pm; 
                 rr:objectMap ?om; 
                 rr:graphMap ?gm. 
        }
        INSERT {
            ?tm rr:predicateObjectMap [
                rr:predicateMap ?pm; 
                rr:objectMap ?om; 
                rr:graphMap ?gm
            ]
        }
        WHERE {
            ?tm rr:predicateObjectMap ?pom. 
            ?pom rr:predicateMap ?pm; 
                 rr:objectMap ?om. 
            
            OPTIONAL {
                ?pom rr:graphMap ?gm. 
            }
        }

        ",
    )?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("multi_pom_to_singleton.ttl")).unwrap();
    }

    Ok(store)
}

fn replace_self_ref_maps(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?om rr:parentTriplesMap ?ptm 
        }
        INSERT {
            ?om rml:reference ?ref ; 
                rr:template ?template ;
                rr:constant ?const; 
                rr:termType rr:IRI 
        }
        WHERE {
            ?om rr:parentTriplesMap ?ptm . 
            ?ptm rr:subjectMap ?sm . 
            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            FILTER NOT EXISTS {
                ?om rr:joinCondition ?jc
            }
        }
        
        ",
    )?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("ref_obj_map_replace.ttl")).unwrap();
    }
    Ok(store)
}

fn duplicate_tm_with_mulitple_poms(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom
        }
        INSERT {
            [] a rr:TriplesMap; 
                rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom 

        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom 
        }
        ",
    )?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("duplicate_tm_multi_pom.ttl")).unwrap();
    }

    Ok(store)
}

fn ensure_tm_smgm_or_pomgm(store: &Store) -> Result<&Store, OxigraphError> {
    store.update(
        "
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom . 
            ?pom rr:graphMap ?pom_gm .
        }
        INSERT {
            [] a rr:TriplesMap; 
               rml:logicalSource ?ls; 
               rr:subjectMap [
                   rr:reference ?ref; 
                   rr:template ?template; 
                   rr:constant ?const; 
                   rr:termType ?ttype; 
                   rr:graphMap ?pom_gm 
               ]; 
               rr:predicateObjectMap ?pom. 
        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom .

            ?pom rr:graphMap ?pom_gm . 

            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            OPTIONAL { ?sm rr:termType ?ttype. }
        }
        ",
    )?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("push_graph_map.ttl")).unwrap();
    }
    Ok(store)
}

pub fn triples_maps_one_graph_map(store: &Store) -> Result<&Store, OxigraphError> {
    let query = "
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom .
            ?sm rr:graphMap ?sm_gm .
        }
        INSERT {
            [] a rr:TriplesMap; 
               rml:logicalSource ?ls; 
               rr:subjectMap [
                   rr:reference ?ref; 
                   rr:template ?template; 
                   rr:constant ?const; 
                   rr:termType ?ttype; 
                   rr:graphMap ?sm_gm
               ]; 
               rr:predicateObjectMap ?pom. 
        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom .
            ?sm rr:graphMap ?sm_gm . 
            ?sm rr:graphMap ?sm_gm2 .
            FILTER( ?sm_gm != ?sm_gm2 )

            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            OPTIONAL { ?sm rr:termType ?ttype. }
        }
        ";

    store.update(query)?;

    if cfg!(debug_assertions) {
        crate::io::dump_oxigraph_store(store, Path::new("tm_one_gm.ttl")).unwrap();
    }
    Ok(store)
}

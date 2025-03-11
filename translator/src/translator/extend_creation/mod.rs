use std::rc::Rc;

use anyhow::Result;
use operator::{Function, TermType};
use oxigraph::model::{NamedOrBlankNodeRef, Quad, SubjectRef, TermRef};
use oxigraph::store::{StorageError, Store};

use super::data::QueryAttrMap;
use super::util::termref_to_literal;
use crate::error::oxigraph::OxigraphErrorKind;
use crate::translator::util::get_object;
use crate::FromVocab;

pub fn create_extend_function(
    term_map_subj: SubjectRef,
    store: &Store,
    query_attr_map: &QueryAttrMap,
) -> Result<Function> {
    //CHANGE THIS
    let mut result = Function::Nop;

    // Handle rr:constant
    if let Ok(constant_o) = get_object(
        term_map_subj,
        vocab::r2rml::PROPERTY::CONSTANT.to_named_node().as_ref(),
        store,
    ) {
        let term_type = match constant_o.as_ref() {
            TermRef::NamedNode(_) => Ok(TermType::IRI),
            TermRef::Literal(_) => Ok(TermType::Literal),
            var => {
                Err(OxigraphErrorKind::GenericError(format!(
                    "object node of the rr:constant cannot be a blanknode {}",
                    var
                )))
            }
        }?;

        return Ok(Function::TypedConstant {
            value: constant_o.to_string(),
            term_type,
        }
        .into());
    }

    // Handle rml:reference
    if let Ok(reference_o) = get_object(
        term_map_subj,
        vocab::rml::PROPERTY::REFERENCE.to_named_node().as_ref(),
        store,
    ) {
        let query = termref_to_literal(reference_o.as_ref())?.value();
        let value = query_attr_map
            .get(query)
            .ok_or(OxigraphErrorKind::GenericError(format!(
                "reference cannot be a named node or a blank node {}",
                reference_o.as_ref()
            )))?
            .to_string();
        result = Function::Reference { value };

    // Handle rr:template
    } else if let Ok(template_o) = get_object(
        term_map_subj,
        vocab::r2rml::PROPERTY::TEMPLATE.to_named_node().as_ref(),
        store,
    ) {
        let template_string = termref_to_literal(template_o.as_ref())?.value();
        let split_template = split_template(template_string);

        for s_i in split_template {
            let (is_query, extracted_string) = get_is_query_str_pair(&s_i);
            let mut function: Function;
            if is_query {
                let value =
                    query_attr_map.get(extracted_string).unwrap().to_string();
                function = Function::Reference { value };
            } else {
                function = Function::TypedConstant {
                    value:     extracted_string.to_string(),
                    term_type: TermType::Literal,
                };
            }

            result = Function::Concatenate {
                left_value:  Rc::new(result),
                separator:   "".to_string(),
                right_value: function.into(),
            };
        }
    }

    if is_term_type(
        term_map_subj,
        vocab::r2rml::CLASS::BLANKNODE
            .to_named_node()
            .as_ref()
            .into(),
        store,
    ) {
        result = Function::BlankNode {
            inner_function: Rc::new(result),
        };
    } else if is_term_type(
        term_map_subj,
        vocab::r2rml::CLASS::IRI.to_named_node().as_ref().into(),
        store,
    ) {
        result = Function::Iri {
            base_iri:       None,
            inner_function: Rc::new(result),
        }
    }

    let mut dtype_function = None;
    if let Ok(term_type) = get_object(
        term_map_subj.into(),
        vocab::r2rml::PROPERTY::DATATYPE.to_named_node().as_ref(),
        store,
    ) {
        dtype_function = Some(Rc::new(Function::Constant {
            value: term_type.to_string(),
        }));
    }
    result = Function::Literal {
        inner_function: Rc::new(result),
        dtype_function,
        langtype_function: None,
    };

    Ok(result)
}

fn is_term_type(term: SubjectRef, term_type_o: TermRef, store: &Store) -> bool {
    store
        .quads_for_pattern(
            Some(term),
            Some(vocab::r2rml::PROPERTY::TERMTYPE.to_named_node().as_ref()),
            Some(term_type_o),
            None,
        )
        .next()
        .is_some()
}

fn get_is_query_str_pair(string: &str) -> (bool, &str) {
    if string.starts_with('{') && string.ends_with('}') {
        (true, &string[1..string.len() - 1])
    } else {
        (false, string)
    }
}

fn split_template(template_string: &str) -> Vec<String> {
    let mut is_escape;
    let mut result = vec![];
    let mut current_buf = String::new();

    let mut chars = template_string.chars();

    while let Some(c) = chars.next() {
        is_escape = c == '\\';
        if is_escape {
            current_buf.push(c);
            if let Some(c) = chars.next() {
                current_buf.push(c);
            }
        } else if c == '{' {
            result.push(current_buf.clone());
            current_buf.clear();
            current_buf.push(c);
        } else if c == '}' {
            current_buf.push(c);
            result.push(current_buf.clone());
            current_buf.clear();
        } else {
            current_buf.push(c);
        }
    }
    if !current_buf.is_empty() {
        result.push(current_buf);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_template_test() -> Result<()> {
        let template = "http://example/{id}/\\{{blah}\\}";
        let split_vec = split_template(template);

        assert_eq!(split_vec[0], "http://example/");
        assert_eq!(split_vec[1], "{id}");
        assert_eq!(split_vec[2], "/\\{");
        assert_eq!(split_vec[3], "{blah}");
        assert_eq!(split_vec[4], "\\}");

        Ok(())
    }
}

use std::cmp::{PartialOrd, Ordering};
use rustling::{ParserMatch, ParsedNode, Candidate, MaxElementTagger, Value, Range};
use rustling_ontology_values::ParsingContext;
use rustling_ontology_values::dimension::{Dimension};
use rustling_ontology_values::output::OutputKind;

use mapper;

pub struct CandidateTagger<'a, C: ParsingContext<Dimension> + 'a> {
    pub output_kind_filter: &'a [OutputKind],
    pub context: &'a C,
    pub resolve_all_candidates: bool,
}

impl<'a, C: ParsingContext<Dimension>> MaxElementTagger<Dimension> for CandidateTagger<'a, C> {
    type O = Option<C::O>;

    fn tag(&self,
           mut candidates: Vec<(ParsedNode<Dimension>, ParserMatch<Dimension>)>)
        -> Vec<Candidate<Dimension, Option<C::O>>> {

        // The filter is an OutputKind vector

        // Update the candidate Dimension values, specifically for Datetime
        // values, which will be tagged with a specific subtype or with Datetime.
        // This is necessary to filter candidates acc. to the OutputKind filter, and to
        // later propagate the info of Datetime subtype to the Output value.
        // => parsed_node.value and parser_match.value are a Dimension(dimension_value)

        for (ref mut parsed_node, ref mut parser_match) in &mut candidates {
            mapper::map_dimension(&mut parsed_node.value, self.output_kind_filter);
            mapper::map_dimension(&mut parser_match.value, self.output_kind_filter);
        }

        // 1. Filtering and priorisation of candidates among OutputKinds, based on the filter:
        // - presence: candidate is valid if its dimension matches an OutputKind present in the
        // filter
        // - order: candidate associated with position of OutputKind in the filter
        let mut candidates = candidates.into_iter()
            .filter_map(|(parsed_node, parser_match)| {
                if parsed_node.value.is_too_ambiguous() { None }
                else {
                    self.output_kind_filter
                        .iter()
                        .rev()
                        // Keep candidates whose Dimension(dimension_value) matches an OutputKind
                        // from the filter.
                        .position(|output_kind| output_kind.match_dim(&parsed_node.value))
                        .map(|position| (parsed_node, parser_match, position))
                }
            })
            .collect::<Vec<_>>();

        // 2. Priorisation intra OutputKind - Use probas from training, and many other things
        // like match length etc.
        candidates.sort_by(|a, b|{
            a.1.byte_range.len().cmp(&b.1.byte_range.len())
                .then_with(|| {
                    a.1.byte_range.0.cmp(&b.1.byte_range.0)
                })
                .then_with(|| {
                    a.2.cmp(&b.2)
                })
                .then_with(|| {
                    if a.1.value.kind() == b.1.value.kind() {
                        a.1.probalog
                            .partial_cmp(&b.1.probalog)
                            .unwrap_or(Ordering::Equal)
                    } else {
                        Ordering::Equal
                    }
                })
                .then_with(|| {
                    b.1.parsing_tree_height.cmp(&a.1.parsing_tree_height)
                })
                .then_with(|| {
                    b.1.parsing_tree_num_nodes.cmp(&a.1.parsing_tree_num_nodes)
                })
        });

        let mut selected_ranges: Vec<Range> = vec![];

        candidates.into_iter().rev().map(|c| {
            if selected_ranges.iter().all(|a| a.is_disjoint(&c.1.byte_range)) {
                let resolved_value = self.context.resolve(&c.1.value);
                if resolved_value.is_some() {
                    selected_ranges.push(c.1.byte_range);
                    return Candidate {
                        node: c.0,
                        match_:  ParserMatch { 
                            byte_range: c.1.byte_range, 
                            char_range: c.1.char_range,
                            parsing_tree_height: c.1.parsing_tree_height,
                            parsing_tree_num_nodes: c.1.parsing_tree_num_nodes,
                            value: resolved_value, 
                            probalog: c.1.probalog, 
                            latent: c.1.latent 
                        }, 
                        tagged: true 
                    }
                }
            }
            let resolved_value = if self.resolve_all_candidates {
                self.context.resolve(&c.1.value)
            } else {
                None
            };
            Candidate {
                node: c.0,
                match_:  ParserMatch { 
                    byte_range: c.1.byte_range, 
                    char_range: c.1.char_range,
                    parsing_tree_height: c.1.parsing_tree_height,
                    parsing_tree_num_nodes: c.1.parsing_tree_num_nodes,
                    value: resolved_value, 
                    probalog: c.1.probalog, 
                    latent: c.1.latent 
                }, 
                tagged: false 
            }
        })
        .collect()
    }
}
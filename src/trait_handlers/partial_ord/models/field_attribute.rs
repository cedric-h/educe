use super::super::super::create_path_string_from_lit_str;

use crate::panic;
use crate::syn::{Attribute, Lit, Meta, NestedMeta};
use crate::Trait;

#[derive(Debug, Clone)]
pub struct FieldAttribute {
    pub ignore: bool,
    pub compare_method: Option<String>,
    pub compare_trait: Option<String>,
    pub rank: isize,
}

#[derive(Debug, Clone)]
pub struct FieldAttributeBuilder {
    pub enable_ignore: bool,
    pub enable_impl: bool,
    pub rank: isize,
    pub enable_rank: bool,
}

impl FieldAttributeBuilder {
    pub fn from_partial_ord_meta(&self, meta: &Meta) -> FieldAttribute {
        let mut ignore = false;

        let mut compare_method = None;
        let mut compare_trait = None;

        let mut rank = self.rank;

        let correct_usage_for_partial_ord_attribute = {
            let mut usage = vec![];

            if self.enable_ignore {
                usage.push(stringify!(#[educe(PartialOrd = false)]));
                usage.push(stringify!(#[educe(PartialOrd(false))]));
            }

            usage
        };

        let correct_usage_for_ignore = {
            let usage = vec![stringify!(#[educe(PartialOrd(ignore))])];

            usage
        };

        let correct_usage_for_impl = {
            let usage = vec![
                stringify!(#[educe(PartialOrd(method = "path_to_method"))]),
                stringify!(#[educe(PartialOrd(trait = "path_to_trait"))]),
                stringify!(#[educe(PartialOrd(trait = "path_to_trait", method = "path_to_method_in_trait"))]),
                stringify!(#[educe(PartialOrd(method("path_to_method")))]),
                stringify!(#[educe(PartialOrd(trait("path_to_trait")))]),
                stringify!(#[educe(PartialOrd(trait("path_to_trait"), method("path_to_method_in_trait")))]),
            ];

            usage
        };

        let correct_usage_for_rank = {
            let usage = vec![
                stringify!(#[educe(PartialOrd(rank = priority_value))]),
                stringify!(#[educe(PartialOrd(rank(priority_value)))]),
            ];

            usage
        };

        let mut rank_is_set = false;

        match meta {
            Meta::List(list) => {
                let mut ignore_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "ignore" => {
                                    if !self.enable_ignore {
                                        panic::unknown_parameter("PartialOrd", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::Word(_) => {
                                            if ignore_is_set {
                                                panic::reset_parameter(meta_name.as_str());
                                            }

                                            ignore_is_set = true;

                                            ignore = true;
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_ignore,
                                        ),
                                    }
                                }
                                "method" => {
                                    if !self.enable_impl {
                                        panic::unknown_parameter("PartialOrd", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if compare_method.is_some() {
                                                                panic::reset_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }

                                                            let s =
                                                                create_path_string_from_lit_str(s);

                                                            if let Some(s) = s {
                                                                compare_method = Some(s);
                                                            } else {
                                                                panic::empty_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(
                                                            meta_name.as_str(),
                                                            &correct_usage_for_impl,
                                                        ),
                                                    },
                                                    _ => panic::parameter_incorrect_format(
                                                        meta_name.as_str(),
                                                        &correct_usage_for_impl,
                                                    ),
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if compare_method.is_some() {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    let s = create_path_string_from_lit_str(s);

                                                    if let Some(s) = s {
                                                        compare_method = Some(s);
                                                    } else {
                                                        panic::empty_parameter(meta_name.as_str());
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_impl,
                                                ),
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_impl,
                                        ),
                                    }
                                }
                                "trait" => {
                                    if !self.enable_impl {
                                        panic::unknown_parameter("PartialOrd", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if compare_trait.is_some() {
                                                                panic::reset_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }

                                                            let s =
                                                                create_path_string_from_lit_str(s);

                                                            if let Some(s) = s {
                                                                compare_trait = Some(s);
                                                            } else {
                                                                panic::empty_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(
                                                            meta_name.as_str(),
                                                            &correct_usage_for_impl,
                                                        ),
                                                    },
                                                    _ => panic::parameter_incorrect_format(
                                                        meta_name.as_str(),
                                                        &correct_usage_for_impl,
                                                    ),
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if compare_trait.is_some() {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    let s = create_path_string_from_lit_str(s);

                                                    if let Some(s) = s {
                                                        compare_trait = Some(s);
                                                    } else {
                                                        panic::empty_parameter(meta_name.as_str());
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_impl,
                                                ),
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_impl,
                                        ),
                                    }
                                }
                                "rank" => {
                                    if !self.enable_rank {
                                        panic::unknown_parameter("PartialOrd", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Int(i) => {
                                                            if rank_is_set {
                                                                panic::reset_parameter(
                                                                    meta_name.as_str(),
                                                                );
                                                            }

                                                            let i = i.value();

                                                            rank_is_set = true;

                                                            if i > isize::max_value() as u64 {
                                                                rank = isize::max_value();
                                                            } else {
                                                                rank = i as isize;
                                                            }
                                                        }
                                                        _ => panic::parameter_incorrect_format(
                                                            meta_name.as_str(),
                                                            &correct_usage_for_rank,
                                                        ),
                                                    },
                                                    _ => panic::parameter_incorrect_format(
                                                        meta_name.as_str(),
                                                        &correct_usage_for_rank,
                                                    ),
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Int(i) => {
                                                    if rank_is_set {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    let i = i.value();

                                                    rank_is_set = true;

                                                    if i > isize::max_value() as u64 {
                                                        rank = isize::max_value();
                                                    } else {
                                                        rank = i as isize;
                                                    }
                                                }
                                                _ => panic::parameter_incorrect_format(
                                                    meta_name.as_str(),
                                                    &correct_usage_for_rank,
                                                ),
                                            }
                                        }
                                        _ => panic::parameter_incorrect_format(
                                            meta_name.as_str(),
                                            &correct_usage_for_rank,
                                        ),
                                    }
                                }
                                _ => panic::unknown_parameter("PartialOrd", meta_name.as_str()),
                            }
                        }
                        _ => panic::attribute_incorrect_format(
                            "PartialOrd",
                            &correct_usage_for_partial_ord_attribute,
                        ),
                    }
                }
            }
            _ => panic::attribute_incorrect_format(
                "PartialOrd",
                &correct_usage_for_partial_ord_attribute,
            ),
        }

        if compare_trait.is_some() {
            if compare_method.is_none() {
                compare_method = Some("partial_cmp".to_string());
            }
        }

        if ignore && rank_is_set {
            panic::ignore_ranked_field();
        }

        FieldAttribute {
            ignore,
            compare_method,
            compare_trait,
            rank,
        }
    }

    pub fn from_attributes(self, attributes: &[Attribute], traits: &[Trait]) -> FieldAttribute {
        let mut result = None;

        for attribute in attributes.iter() {
            let meta = attribute.parse_meta().unwrap();

            let meta_name = meta.name().to_string();

            match meta_name.as_str() {
                "educe" => match meta {
                    Meta::List(list) => {
                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.name().to_string();

                                    let t = Trait::from_str(meta_name);

                                    if let Err(_) = traits.binary_search(&t) {
                                        panic::trait_not_used(t.as_str());
                                    }

                                    if t == Trait::PartialOrd {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_partial_ord_meta(&meta));
                                    }
                                }
                                _ => panic::educe_format_incorrect(),
                            }
                        }
                    }
                    _ => panic::educe_format_incorrect(),
                },
                _ => (),
            }
        }

        result.unwrap_or(FieldAttribute {
            ignore: false,
            compare_method: None,
            compare_trait: None,
            rank: self.rank,
        })
    }
}
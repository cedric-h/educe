use std::str::FromStr;
use std::fmt::Write;

use super::super::TraitHandler;
use super::models::{TypeAttributeBuilder, FieldAttributeBuilder};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data, Generics};

pub struct CloneStructHandler;

impl TraitHandler for CloneStructHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let type_attribute = TypeAttributeBuilder {
            enable_bound: true,
        }.from_clone_meta(meta);

        let bound = type_attribute.bound.into_punctuated_where_predicates_by_generic_parameters(&ast.generics.params);

        let mut clone_tokens = TokenStream::new();
        let mut clone_from_tokens = TokenStream::new();

        if let Data::Struct(data) = &ast.data {
            let mut field_attributes = Vec::new();
            let mut field_names = Vec::new();
            let mut has_custom_clone_method = false;

            for (index, field) in data.fields.iter().enumerate() {
                let field_attribute = FieldAttributeBuilder {
                    enable_clone: true,
                }.from_attributes(&field.attrs, traits);

                let field_name = if let Some(ident) = field.ident.as_ref() {
                    ident.to_string()
                } else {
                    format!("{}", index)
                };

                if field_attribute.clone_method.is_some() {
                    has_custom_clone_method = true;
                }

                field_attributes.push(field_attribute);
                field_names.push(field_name);
            }

            if !has_custom_clone_method && traits.contains(&Trait::Copy) {
                clone_tokens.extend(quote!(*self));

                let mut clone_from = String::new();

                for (index, field_attribute) in field_attributes.into_iter().enumerate() {
                    let field_name = &field_names[index];

                    clone_from.write_fmt(format_args!("core::clone::Clone::clone_from(&mut self.{field_name}, &_source.{field_name});", field_name = field_name)).unwrap();
                }

                clone_from_tokens.extend(TokenStream::from_str(&clone_from).unwrap());
            } else {
                let (is_unit, is_tuple) = {
                    if let Some(field) = data.fields.iter().next() {
                        if let Some(_) = field.ident {
                            (false, false)
                        } else {
                            (false, true)
                        }
                    } else {
                        (true, true)
                    }
                };

                let mut clone = ast.ident.to_string();
                let mut clone_from = String::new();

                if is_tuple {
                    clone.push('(');

                    for (index, field_attribute) in field_attributes.into_iter().enumerate() {
                        let field_name = &field_names[index];

                        // TODO
                        let clone_trait = field_attribute.clone_trait;
                        let clone_method = field_attribute.clone_method;

                        clone.write_fmt(format_args!("core::clone::Clone::clone(&self.{field_name}),", field_name = field_name)).unwrap();
                        clone_from.write_fmt(format_args!("core::clone::Clone::clone_from(&mut self.{field_name}, &_source.{field_name});", field_name = field_name)).unwrap();
                    }

                    clone.push(')');
                } else {
                    clone.push('{');

                    for (index, field_attribute) in field_attributes.into_iter().enumerate() {
                        let field_name = &field_names[index];

                        // TODO
                        let clone_trait = field_attribute.clone_trait;
                        let clone_method = field_attribute.clone_method;

                        clone.write_fmt(format_args!("{field_name}: core::clone::Clone::clone(&self.{field_name}),", field_name = field_name)).unwrap();
                        clone_from.write_fmt(format_args!("core::clone::Clone::clone_from(&mut self.{field_name}, &_source.{field_name});", field_name = field_name)).unwrap();
                    }

                    clone.push('}');
                }

                if is_unit {
                    let ident = &ast.ident;

                    clone_tokens.extend(quote!(#ident));
                } else {
                    clone_tokens.extend(TokenStream::from_str(&clone).unwrap());
                }
                clone_from_tokens.extend(TokenStream::from_str(&clone_from).unwrap());
            }
        }

        let ident = &ast.ident;

        let mut generics_cloned: Generics = ast.generics.clone();

        let where_clause = generics_cloned.make_where_clause();

        for where_predicate in bound {
            where_clause.predicates.push(where_predicate);
        }

        let (impl_generics, ty_generics, where_clause) = generics_cloned.split_for_impl();

        let compare_impl = quote! {
            impl #impl_generics core::clone::Clone for #ident #ty_generics #where_clause {
                fn clone(&self) -> Self {
                    #clone_tokens
                }

                fn clone_from(&mut self, _source: &Self) {
                    #clone_from_tokens
                }
            }
        };

        tokens.extend(compare_impl);
    }
}
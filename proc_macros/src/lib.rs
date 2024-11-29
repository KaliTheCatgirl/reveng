#[proc_macro_derive(FromBytes)]
pub fn from_bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;

    let generics = from_bytes::add_trait_bounds(input.generics);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let size_expression = from_bytes::total_size_expression(&input.data);

    let field_init = from_bytes::field_initialisations(&input.data);

    let output = quote::quote! {
        impl #impl_generics reveng::from_bytes::FromBytes for #name #type_generics #where_clause {
            const SIZE: usize = #size_expression;
            fn from_bytes(data: [u8; Self::SIZE], endianness: reveng::endianness::Endianness) -> Self {
                Self #field_init
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Readable)]
pub fn readable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;

    let generics = readable::add_trait_bounds(input.generics);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let field_init = readable::field_initialisations(&input.data);

    let output = quote::quote! {
        impl #impl_generics reveng::read::Readable for #name #type_generics #where_clause {
            fn read_from<R: std::io::Read>(mut read: R, endianness: reveng::endianness::Endianness) -> std::io::Result<Self> {
                Ok(Self #field_init)
            }
        }
    };
    output.into()
}

mod from_bytes {
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::{parse_quote, spanned::Spanned, Data, Fields, GenericParam, Generics};

    pub fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(parse_quote!(reveng::from_bytes::FromBytes));
            }
        }
        generics
    }

    pub fn total_size_expression(data: &Data) -> TokenStream {
        match data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    let recurse = fields.named.iter().map(|field| {
                        let field_type = &field.ty;
                        quote_spanned! {field.span() => <#field_type as reveng::from_bytes::FromBytes>::SIZE}
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(fields) => {
                    let recurse = fields.unnamed.iter().map(|field| {
                        let field_type = &field.ty;
                        quote_spanned! {field.span() => <#field_type as reveng::from_bytes::FromBytes>::SIZE}
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => quote!(0),
            },
            _ => unimplemented!(),
        }
    }

    pub fn field_initialisations(data: &Data) -> TokenStream {
        match data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    let mut output = TokenStream::new();
                    let mut old_field_offset = quote!(0);
                    for (field_name, field_type, new_field_size, span) in fields.named.iter().map(|field| {
                        let field_type = &field.ty;
                        (&field.ident, field_type, quote_spanned! {field.span()=> <#field_type as reveng::from_bytes::FromBytes>::SIZE}, field.span())
                    }) {
                        let mut new_field_offset = old_field_offset.clone();
                        new_field_offset.extend(quote_spanned!(span=> + #new_field_size));

                        output.extend(quote_spanned! {
                            span=> #field_name: <#field_type as reveng::from_bytes::FromBytes>::from_bytes(
                                <[u8; #new_field_size] as TryFrom<&[u8]>>::try_from(&data[#old_field_offset..#new_field_offset]).unwrap(), endianness),
                        });

                        old_field_offset = new_field_offset;
                    }
                    quote!({#output})
                }
                Fields::Unnamed(fields) => {
                    let mut output = TokenStream::new();
                    let mut old_field_offset = quote!(0);
                    for (field_type, new_field_size, span) in fields.unnamed.iter().map(|field| {
                        let field_type = &field.ty;
                        (field_type, quote_spanned! {field.span()=> <#field_type as reveng::from_bytes::FromBytes>::SIZE}, field.span())
                    }) {
                        let mut new_field_offset = old_field_offset.clone();
                        new_field_offset.extend(quote_spanned!(span=> + #new_field_size));

                        output.extend(quote_spanned! {
                            span=> <#field_type as reveng::from_bytes::FromBytes>::from_bytes(
                                <[u8; #new_field_size] as TryFrom<&[u8]>>::try_from(&data[#old_field_offset..#new_field_offset]).unwrap(), endianness),
                        });

                        old_field_offset = new_field_offset;
                    }
                    quote!((#output))
                }
                Fields::Unit => TokenStream::new(),
            },
            _ => unimplemented!(),
        }
    }
}

mod readable {
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::{parse_quote, spanned::Spanned, Data, Fields, GenericParam, Generics};

    pub fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(parse_quote!(reveng::read::Readable));
            }
        }
        generics
    }

    pub fn field_initialisations(data: &Data) -> TokenStream {
        match data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    let recurse = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        quote_spanned! { field.span()=> #field_name: #field_type::read_from(&mut read, endianness)?, }
                    });
                    quote! {{ #(#recurse)* }}
                }
                Fields::Unnamed(fields) => {
                    let recurse = fields.unnamed.iter().map(|field| {
                        let field_type = &field.ty;

                        quote_spanned! { field.span()=> #field_type::read_from(&mut read, endianness)?, }
                    });
                    quote!((#(#recurse)*))
                }
                Fields::Unit => TokenStream::new(),
            },
            _ => unimplemented!(),
        }
    }
}

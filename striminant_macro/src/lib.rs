
use proc_macro::TokenStream;
// this hacking was necessary to calculate the discriminant in case it wasn't explicitly specified
use quote::{__private::{Span, TokenStream as TokenStream2}, quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, 
    Data::{Enum, Struct, Union},
    DeriveInput, Expr,  Fields, Lit, LitByte, LitChar, LitInt,
};

struct ResolvedVariant {
    attrs: TokenStream2,
    resolved_discriminant: u8,
    raw_discriminant: Option<TokenStream2>,
    ident: TokenStream2,
}

fn get_value_from_literal(lit: &Lit) -> u8 {
    match lit {
        Lit::Byte(val) => val.value(),
        Lit::Char(val) => val.value() as u8,
        Lit::Int(val) => val.to_string().parse::<u8>().unwrap(),
        _ => panic!("cannot get the value of the specified literal"),   
    }
}


impl ToTokens for ResolvedVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {        
        // add the attributes
        tokens.append_all(self.attrs.to_owned());
        
        // add identifier
        tokens.append_all(self.ident.to_owned());

        // add discriminant (if explicitly specified)
        if let Some(discriminant) = &self.raw_discriminant {
            tokens.append_all("=".parse::<TokenStream2>());
            tokens.append_all(discriminant.to_owned());
        }
    }
}



/// Builds upon strum_macro to enable disriminant values to be used (after being parsed into
/// ASCII) as values for deriving string-enum traits.
/// 
/// Under the hood, the macro 
#[proc_macro_attribute]
pub fn striminant(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let enum_attributes = input.attrs;
    let enum_name = input.ident;

    let variants: Vec<ResolvedVariant> = match input.data {
        Struct(_) => panic!("cannot use macro on struct"),
        Union(_) => panic!("cannot use macro on union"),
        Enum(data) => {
            let mut last_discriminant: u8 = 0;

            data.variants.iter().map(
                |variant| {
                if variant.fields != Fields::Unit {
                    panic!("enum cannot contain tuple or struct variants");
                }

                let attrs = variant.attrs.to_owned();
                let attrs = quote! (#(#attrs)*);
                let has_strum_specifier = attrs.to_string().replace(" ", "").contains("#[strum(serialize");



                let ident = variant.ident.to_owned().into_token_stream();

                let (discriminant, raw_discriminant) = match &variant.discriminant {
                    Some((_, expr)) => {
                        let raw_discriminant = expr.to_token_stream();

                        let discriminant = match expr {
                            Expr::Lit(literal) => match literal.lit {
                                 Lit::Byte(_) | Lit::Char(_) | Lit::Int(_) => {
                                    literal.lit.to_owned()
                                }
                                _ => panic!("cannot coerce discriminant value to a character"),
                            },
                            _ => panic!("unable to parse discriminant value; value must be a byte, char or integer literal"),
                        };

                        last_discriminant = get_value_from_literal(&discriminant);
                        (&last_discriminant, Some(raw_discriminant))
                    }
                    None => {
                        last_discriminant += 1;
                        (&last_discriminant, None)
                    }
                };
                
                let discriminant_char_value = (*discriminant as char).to_string();

                let attrs = if !has_strum_specifier {
                    quote!{
                        #attrs
                        #[strum(serialize = #discriminant_char_value)]
                    }
                } else {attrs};



                ResolvedVariant {
                    attrs,
                    resolved_discriminant: *discriminant,
                    ident: ident.to_owned(),
                    raw_discriminant
                }

                }
            ).collect()

            // for variant in data.variants {
            //     if variant.fields != Fields::Unit {
            //         panic!("enum cannot contain tuple or struct variants");
            //     }

            //     let attrs = variant.attrs;
            //     let attrs = quote! (#(#attrs),*);

            //     let has_strum_specifier = attrs.to_string().contains("#[strum(serialize");
            //     let ident = variant.ident.to_string();

            //     let discriminant = match variant.discriminant {
            //         Some((_, expr)) => {
            //             let discriminant = match expr {
            //                 Expr::Lit(literal) => match literal.lit {
            //                      Lit::Byte(_) | Lit::Char(_) | Lit::Int(_) => {
            //                         literal.lit
            //                     }
            //                     _ => panic!("cannot coerce discriminant value to a character"),
            //                 },
            //                 _ => panic!("unable to parse discriminant value; value must be a byte, char or integer literal"),
            //             };

            //             last_discriminant = discriminant;
            //             &last_discriminant
            //         }
            //         None => {
            //             last_discriminant = get_successor(&last_discriminant);
            //             &last_discriminant
            //         }
            //     };

            // }
        }
    };


    let output = TokenStream::from(quote! {
        #(#enum_attributes)*
        enum #enum_name {
            #(#variants),*
        }
    });

    println!("{}", output);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_works() {}
}

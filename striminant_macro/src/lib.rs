mod parse_attribute_arguments;

use crate::parse_attribute_arguments::{parse_attribute_arguments, OutputType};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
// this hacking was necessary to calculate the discriminant in case it wasn't explicitly specified
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input,
    Data::{Enum, Struct, Union},
    DeriveInput, Expr, Fields, Lit,
};

struct ParsedVariant {
    attrs: TokenStream2,
    discriminant: Option<TokenStream2>,
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

impl ToTokens for ParsedVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // add the attributes
        tokens.append_all(self.attrs.to_owned());

        // add identifier
        tokens.append_all(self.ident.to_owned());

        // add discriminant (if explicitly specified)
        if let Some(discriminant) = &self.discriminant {
            tokens.append_all("=".parse::<TokenStream2>());
            tokens.append_all(discriminant.to_owned());
        }
    }
}

/// Builds upon strum_macro to enable disriminant values to be used (after being parsed into
/// ASCII or just as numbers) as values for deriving string-enum traits.
///
/// Under the hood, the macro produces #[strum(serialize)] macros for each variant,
/// using the discriminant's value.
///
/// It is necessary to include this macro before your strum derive macro.
///
/// Currently, only literals can be used as discriminant values with this macro.
///
///
/// # Examples:
///
/// Example with char output
/// ```
/// use striminant_macro::striminant;
/// use strum_macros::IntoStaticStr;
///
/// // use the discriminants as character codes but disallow a discriminant
/// // value of b'6'
/// #[striminant(output = "char", except = [b'6'])]
/// #[derive(PartialEq, IntoStaticStr)]
/// // Otherwise the compiler errors that the variants must be isize. Currently
/// // there is no way around this while using this macro, however, this might
/// // change in the future.
/// #[repr(u8)]
/// enum Digit {
///     // 49 is the char code for '1'
///     One = 49,
///     Two = b'2',
///     #[strum(serialize = "THREE")]
///     Three,
///     Four = b'4',
///     Five,
/// }
///
/// let one: &'static str = Digit::One.into();
/// let two: &'static str = Digit::Two.into();
/// let three: &'static str = Digit::Three.into();
/// let four: &'static str = Digit::Four.into();
/// let five: &'static str = Digit::Five.into();
///
/// assert_eq!(one, "1");
/// assert_eq!(two, "2");
/// assert_eq!(three, "THREE");
/// assert_eq!(four, "4");
/// assert_eq!(five, "5");
/// ```
///
/// Example with num output:
/// ```
/// use striminant_macro::striminant;
/// use strum_macros::IntoStaticStr;
///
/// #[striminant(output = "num", except = [b'6'])]
/// #[derive(PartialEq, IntoStaticStr)]
/// enum Digit {
///     Zero = 0,
///     One,
///     Two,
/// }
///
/// let zero: &'static str = Digit::Zero.into();
/// let one: &'static str = Digit::One.into();
/// let two: &'static str = Digit::Two.into();
///
/// assert_eq!(zero, "0");
/// assert_eq!(one, "1");
/// assert_eq!(two, "2");
/// ```
///
#[proc_macro_attribute]
pub fn striminant(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let config = parse_attribute_arguments(attr_args.into());

    let input_enum = parse_macro_input!(item as DeriveInput);

    let enum_generics = input_enum.generics;
    let enum_visibility = input_enum.vis;
    let enum_attributes = input_enum.attrs;
    let enum_name = input_enum.ident;

    let variants: Vec<ParsedVariant> = match input_enum.data {
        Struct(_) => panic!("cannot use macro on struct"),
        Union(_) => panic!("cannot use macro on union"),
        Enum(data) => {
            let mut last_discriminant: u8 = 0;
            data.variants.iter().map(
                |variant| {
                    if variant.fields != Fields::Unit {
                        panic!("enum cannot contain tuple or struct variants");
                    }

                    let attrs = &variant.attrs;
                    let attrs = quote! (#(#attrs)"\n"*);
                    let has_strum_specifier = attrs.to_string().replace(" ", "").contains("#[strum(serialize");

                    let ident = (&variant.ident).into_token_stream();

                    let (discriminant, raw_discriminant) = match &variant.discriminant {
                        Some((_, expr)) => {
                            let raw_discriminant = expr.to_token_stream();

                            let discriminant = match expr {
                                Expr::Lit(literal) => match literal.lit {
                                    Lit::Byte(_) | Lit::Char(_) | Lit::Int(_) => {
                                        &literal.lit
                                    }
                                    _ => panic!("cannot coerce discriminant value to a character"),
                                },
                                _ => panic!("unable to parse discriminant value; value must be a byte, char or integer literal"),
                            };

                            last_discriminant = get_value_from_literal(&discriminant);

                            if config.exceptions().contains(&last_discriminant) {
                                panic!(
                                    "cannot use discriminant value {} (see 'except' in striminant macro)",
                                    last_discriminant
                                );
                            }

                            (&last_discriminant, Some(raw_discriminant))
                        }
                        None => {
                            last_discriminant += 1;
                            (&last_discriminant, None)
                        }
                    };

                    let discriminant_value = match config.output() {
                        OutputType::Char => (*discriminant as char).to_string(),
                        OutputType::Num => discriminant.to_string(),
                    };


                    let attrs = if !has_strum_specifier {
                        quote!{
                            #attrs
                            #[strum(serialize = #discriminant_value)]
                        }
                    } else {
                        attrs
                    };

                    ParsedVariant {
                        attrs,
                        ident,
                        discriminant: raw_discriminant,
                    }
                }
            ).collect()
        }
    };

    let output = TokenStream::from(quote! {
        #(#enum_attributes)*
        #enum_visibility enum #enum_name #enum_generics {
            #(#variants),*
        }
    });

    output
}

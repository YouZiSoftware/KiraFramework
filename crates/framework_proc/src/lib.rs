extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Expr, Field, Ident, ItemEnum, ItemStruct, parse_macro_input, Path};
use to_snake_case::ToSnakeCase;

fn get_path(crate_name: &str, span: Span) -> Path {
    let found_crate = proc_macro_crate::crate_name(crate_name).unwrap();

    match found_crate {
        FoundCrate::Itself => {
            Path::from(Ident::new("crate", span))
        },
        FoundCrate::Name(name) => {
            Path::from(Ident::new(&name, span))
        }
    }
}

#[proc_macro_derive(OneBotEvent)]
pub fn onebot_event_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let path = get_path("kira_framework", name.span());
    let bevy_path = get_path("bevy_ecs", name.span());

    let expanded = quote! {
        impl #path::network::events::OneBotEventTrait for #name {
            fn send_event(self, world: &mut #bevy_path::world::World) -> anyhow::Result<#bevy_path::event::EventId<#path::network::events::OneBotEventReceiver<Self>>>
            where
                Self: std::marker::Send + std::marker::Sync + Sized + Clone {
                world.send_event(
                    #path::network::events::OneBotEventReceiver::new(self)
                ).ok_or(anyhow::anyhow!("send event({}) error", stringify!(#name)))
            }
            fn to_json(&self) -> anyhow::Result<serde_json::Value> {
                let value = serde_json::to_value(&self)?;
                Ok(value)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OneBotEventsEnum)]
pub fn onebot_events_enum_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemEnum);
    let name = input.ident;

    let path = get_path("kira_framework", name.span());
    let bevy_path = get_path("bevy_ecs", name.span());
    let app_path = get_path("bevy_app", name.span());
    let vars: Vec<Ident> = input.variants.iter().map(|vars| {
        vars.ident.clone()
    }).collect();
    let expanded = quote! {
        impl #path::network::events::OneBotEventsEnumTrait for #name {
            fn from_json(json: String) -> anyhow::Result<Self> where Self: Sized {
                let value: serde_json::Value = serde_json::from_str(json.as_str())?;
                let post_type = value.get("post_type").ok_or(anyhow::anyhow!("post_type not found"))?
                    .as_str().ok_or(anyhow::anyhow!("post_type not str"))?;
                let sub_type = if let Some(sub_type) = value.get("sub_type") {
                    sub_type.as_str().unwrap_or("normal").to_string()
                }else {
                    "normal".to_string()
                };
                let type_value = value.get(format!("{}_type", post_type)).ok_or(anyhow::anyhow!("{}_type not found", post_type))?
                    .as_str().ok_or(anyhow::anyhow!("{}_type not str", post_type))?;
                let post_type = post_type.to_string();
                let type_value = type_value.to_string();
                let mut struct_name_map = std::collections::HashMap::new();
                #(
                struct_name_map.insert((<#vars as #path::network::events::OneBotEventTypeTrait>::get_post_type(), <#vars as #path::network::events::OneBotEventTypeTrait>::get_sub_type(), <#vars as #path::network::events::OneBotEventTypeTrait>::get_type_value()), stringify!(#vars));
                )*
                debug!("find struct: ({}, {:?}, {})", &post_type, &sub_type, &type_value);
                let struct_name = struct_name_map.get(&(post_type, sub_type, type_value)).ok_or(anyhow::anyhow!("struct_name not found"))?.clone();
                match struct_name {
                    #(
                        stringify!(#vars) => {
                            let event: #vars = serde_json::from_str(json.as_str())?;
                            Ok(#name::#vars(event))
                        }
                    ),*
                    _ => {
                        Err(anyhow::anyhow!("from_json({}) error", stringify!(#name)))
                    }
                }
            }
            fn send_event(self, world: &mut #bevy_path::world::World) -> anyhow::Result<()> {
                match self {
                    #(
                        #name::#vars(event) => {
                            world.send_event(#path::network::events::OneBotEventReceiver::new(event)).ok_or(anyhow::anyhow!("send event({}) error", stringify!(#name)))?;
                            Ok(())
                        }
                    ),*
                    _ => {
                        Err(anyhow::anyhow!("send event({}) error", stringify!(#name)))
                    }
                }
            }
            fn add_events(app: &mut #app_path::App) {
                #(
                app.add_event::<#path::network::events::OneBotEventReceiver<#vars>>();
                )*
            }
            fn pretty_debug(&self) -> String {
                match self {
                    #(
                        #name::#vars(event) => {
                            #path::pretty_debug::KiraPrettyDebug::pretty_debug(event)
                        }
                    ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OneBotMessage)]
pub fn onebot_message_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    //大驼峰转下划线
    let name_str = name.to_string().to_snake_case();
    let case_name = Ident::new(&name_str, name.span());

    let path = get_path("kira_framework", name.span());

    let expanded = quote! {
        impl #path::network::message_chain::MessageTrait for #name {
            fn get_type() -> String {
                stringify!(#case_name).to_string()
            }

            fn get_data(&self) -> serde_json::Value {
                serde_json::to_value(&self).unwrap()
            }

            fn as_persistent_string(&self) -> String {
                #path::persistent::AsPersistentStringTrait::as_persistent_string(self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OneBotMessagesEnum)]
pub fn onebot_messages_enum_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemEnum);
    let name = input.ident;

    let vars: Vec<Ident> = input.variants.iter().map(|vars| {
        vars.ident.clone()
    }).collect();

    let vars_case: Vec<String> = input.variants.iter().map(|vars| {
        vars.ident.to_string().to_snake_case()
    }).collect();

    let path = get_path("kira_framework", name.span());

    let expanded = quote! {
        impl #path::network::message_chain::MessagesEnumTrait for #name {
            fn as_persistent_string(message_type: String, data: serde_json::Value) -> anyhow::Result<String> {
                let message_type = message_type.as_str();
                match message_type {
                    #(
                        #vars_case => {
                            let message = serde_json::from_value::<#vars>(data)?;
                            Ok(message.as_persistent_string())
                        }
                    ),*
                    _ => {
                        Err(anyhow::anyhow!("as_persistent_string({}) error", message_type))
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AsPersistentString)]
pub fn as_persistent_string_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    let name = input.ident;

    //大驼峰转下划线
    let name_str = name.to_string().to_snake_case();

    let vars: Vec<Ident> = input.fields.iter().map(|vars| {
        vars.ident.clone().unwrap()
    }).collect();

    let mut format_str = format!("[CQ:{},", name_str);
    for var in &vars {
        format_str += format!("{}={{}},", var.to_string()).as_str();
    }
    format_str = format_str[..format_str.len() - 1].to_string();
    format_str += "]";

    let path = get_path("kira_framework", name.span());

    let expanded = quote! {
        impl #path::persistent::AsPersistentStringTrait for #name {
            fn as_persistent_string(&self) -> String {
                format!(#format_str, #(self.#vars).*)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OneBotAction)]
pub fn onebot_action_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    //大驼峰转下划线
    let name_str = name.to_string().to_snake_case();
    let case_name = Ident::new(&name_str, name.span());

    let path = get_path("kira_framework", name.span());

    let expanded = quote! {
        impl #path::network::actions::OneBotActionTrait for #name {
            fn get_action(&self) -> String {
                stringify!(#case_name).to_string()
            }
            fn get_data(&self) -> serde_json::Value {
                serde_json::to_value(&self).unwrap()
            }
            fn pretty_debug(&self) -> String {
                #path::pretty_debug::KiraPrettyDebug::pretty_debug(self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OneBotActionReturn)]
pub fn onebot_action_return_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let path = get_path("kira_framework", name.span());
    let serde_path = get_path("serde_json", name.span());

    let expanded = quote! {
        impl #path::network::actions::OneBotActionReturnTrait for #name {
            fn from_json(json: #serde_path::Value) -> anyhow::Result<Self> where Self: Sized {
                serde_json::from_value(json).map_err(|e| anyhow::anyhow!("from_json({}) error: {}", stringify!(#name), e))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn kira_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let fields: Vec<&Field> = item.fields.iter().collect();
    let name = item.ident.clone();
    let mut idents = vec![];
    let mut values = vec![];

    for field in fields {
        let ident = field.ident.clone().unwrap();
        idents.push(ident);
        let attrs = field.attrs.clone();
        for attr in attrs {
            if attr.path().is_ident("kira_default") {
                let expr: Expr = attr.parse_args().unwrap();
                values.push(expr);
                break
            }
        }
        let tokens = TokenStream::from(quote! { std::default::Default::default() });
        let expr = parse_macro_input!(tokens as Expr);
        values.push(expr);
    }

    let expanded = quote! {
        #item
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#idents: #values),*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn onebot_event_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(attr as syn::ExprAssign);
    let item = parse_macro_input!(item as ItemStruct);
    let name = item.ident.clone();
    let post_type = expr.left.to_token_stream().to_string();
    let type_value = expr.right.to_token_stream().to_string().replace("\"", "");
    let mut type_value = type_value.as_str();
    let path = get_path("kira_framework", name.span());
    let mut sub_type = "normal";
    if type_value.contains(".") {
        let vec = type_value.split(".").collect::<Vec<&str>>();
        type_value = vec[0];
        sub_type = vec[1];
    }
    let expanded = quote! {
        #item
        impl #path::network::events::OneBotEventTypeTrait for #name {
            fn get_post_type() -> String {
                #post_type.to_string()
            }

            fn get_sub_type() -> String {
                #sub_type.to_string()
            }

            fn get_type_value() -> String {
                #type_value.to_string()
            }
        }
    };
    TokenStream::from(expanded)
}
extern crate proc_macro;

use convert_case::Case;
use convert_case::Casing;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::Ident;
use syn::LitStr;
use syn::Token;

struct PluginInfo {
    name: Ident,
    id: LitStr,
}

impl Parse for PluginInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let id: LitStr = input.parse()?;
        Ok(PluginInfo { name, id })
    }
}

#[proc_macro]
pub fn make_plugin(input: TokenStream) -> TokenStream {
    let PluginInfo { name, id } = parse_macro_input!(input as PluginInfo);
    let plugin_name = name.clone();
    let plugin_snake_case_name = plugin_name.to_string().to_case(Case::Snake);

    let parse_fn_name = format_ident!("parse_{plugin_snake_case_name}_message");
    let attach_fn_name = format_ident!("attach_{plugin_snake_case_name}");

    let expanded = quote! {
        use jarust::prelude::*;

        #[async_trait::async_trait]
        pub trait #name: Attach {
            type Event: Send + Sync + 'static;
            type Handle: From<JaHandle> + std::ops::Deref<Target = JaHandle> + PluginTask;

            fn #parse_fn_name(message: JaResponse) -> JaResult<Self::Event>;

            async fn #attach_fn_name(
                &self,
            ) -> JaResult<(Self::Handle, tokio::sync::mpsc::Receiver<Self::Event>)> {
                let (handle, mut receiver) = self.attach(#id).await?;
                let (tx, rx) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);
                let join_handle = tokio::spawn(async move {
                    while let Some(msg) = receiver.recv().await {
                        let msg = Self::#parse_fn_name(msg)?;
                        let _ = tx.send(msg).await;
                    }
                    Ok::<(), JaError>(())
                });
                let abort_handle = join_handle.abort_handle();
                let mut handle: Self::Handle = handle.into();
                handle.assign_aborts(vec![abort_handle]);
                Ok((handle, rx))
            }
        }
    };

    TokenStream::from(expanded)
}

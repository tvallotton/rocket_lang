# Rocket Lang
Rocket-lang provides a configurable enum type for multi-language rocket applications. 

# LangCode
A request guard corresponding to the [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) code standard. 
Usage example: 
```rust
#[get("/some-path/")]
fn some_path(lang: LangCode) -> Template {
    // we can now choose which template to display
    // based of the user's language preference
    let path = format!("home/{}", LangCode); 
    Template::render(path, json!({}))
}
```

# Config 
The behavior of the enum can be configured with the `Config` structure, which can be attached to a rocket instance. 
When this is not used, its default behavior is to retrieve the language code from the `Accept-Language` header.

## accept_language
If the preferred method for language resolution is the http accept-language header, the qualities for each language can be set like this:
```rust
let config = Config::new(); 
let config[Es] = 1.0; 
let config[En] = 0.5;
```

## url
The guard can also be configured to extract the language code from a fixed position in the path: 
```rust
/// takes the language code from the last path segment:
let config = Config::new().url(-1); 
```

This way the language code can be retrieved from a positional url segment. 
```rust
#[get("see-lang/<_>")]
fn see_lang(lang: LangCode) -> &'static str {
    lang.as_str()
}

```
## custom
If none of the previous approaches suit your needs, you may also use a closure to create a language code from a request: 
```rust
let config = Config::custom(|req: &Request|{
    let lang = from_url(req)?;
    Ok(lang) 
}); 
```



# Composable
Other request guards can consume the structure in their API. Most notably, it can be used by foreign structures to return error messages in multiple languages.

```rust
use rocket_lang::Error; 

// here the error message displayed by
// `Unauthorized` will automatically suit the callers configuration.
#[get("/unauthorized")]
fn unauthorized() -> Unauthorized {
    Unauthorized
}

// A possible implementation of `Unauthorized`
impl<'r, 'o: 'r> Responder<'r, 'o> for Unauthorized {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let lang: LangCode = request
            .try_into()
            .map_err(|x: Error| x.status())?;
        let msg = match lang {
            LangCode::Es => "No autorizado",
            LangCode::Fr => "Non autorisé",
            LangCode::Ge => "Nicht autorisiert"
            _            => "Unauthorized",  
        };
        msg.respond_to(request)
    }
}
```

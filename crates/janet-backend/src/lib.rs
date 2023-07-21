use janetrs::client::{Error, JanetClient};

fn run_janet_code(text: &str) -> Result<(), Error> {
    let client = JanetClient::init_with_default_env()?;

    client.run(text)?;

    let out = client.run("(+ 2 2)")?;

    println!("{out}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = run_janet_code("(print `Hello from Janet!`)");
    }
}

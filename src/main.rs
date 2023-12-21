use entrypoint::prelude::*;

#[entrypoint::entrypoint]
fn entrypoint(args: tera_former::CLIArgs) -> entrypoint::anyhow::Result<()> {
    tera_former::generate(args)
}

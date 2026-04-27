# rust_ibkr_client_portal
<!-- keep thr format -->
## create plain rust project
<!-- ktf -->
<!-- KtF -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- KtF -->
- work on raspi debian
<!-- KtF -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch project_path.md \
&& touch README.md \
&& touch FROM_HERE.md \
&& ln -s README.md README \
&& mkdir -p img \
&& curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg" \
&& echo "[1]: ./img/link_symbol.svg " >>project_path.md \
&& echo "[1]: ./img/link_symbol.svg " >>FROM_HERE.md \
&& echo "[1]: ./img/link_symbol.svg " >>README.md \
&& cargo init \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& rustup override set stable  \
&& cargo clippy \
&& cargo fmt --verbose \
&& cargo fix --workspace \
&& mkdir tests \
&& cargo build \
&& cargo run \
&& cargo run --example example
&& cargo add tokio --features full
```
<!-- KtF -->
<!-- To comply with the format -->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- spell-checker: disable  -->
<!-- keep the format -->
<!-- make folder and download the link sign vai curl -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - **send me a email** -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->

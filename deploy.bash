cargo build --release --target x86_64-unknown-linux-musl
mv target/x86_64-unknown-linux-musl/release/bot orion-bot

orion deploy
# Link Shortener
A link shortener written in Rust using Rocket

# Running Locally

1. Clone the repository
2. Create a `.env` file at the root directory and add your `JWT_SECRET`(String, used to generate user tokens) to it.
```ini
JWT_SECRET = "my_secret"
```
3. Run the server using `cargo run` or build a binary using `cargo b`

If getting a Linker error for `sqlite3.lib`, add the `deps` dir as an env variable 
```bash
$Env:SQLITE3_LIB_DIR = "<path-to-repo>\deps"
```

**Note:** When building a binary, you also need to copy the static directory and put it in the same directory as the binary.

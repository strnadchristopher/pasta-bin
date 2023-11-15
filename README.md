# Pastabin

A Pastebin clone written in Rust using the [Rocket](https://rocket.rs/) web framework. It is:

* **Fast**: Rocket is one of the fastest web frameworks available for Rust.

* **Correct**: No unwraps were used in the making of this program.

* **Secure**: Pastas can be encrypted by calling the "newPastaSecure" endpoint. The encryption key is never stored on the server.

## Usage
Start the server by building the project and running the binary. The server will listen on port 8000 by default.

### Endpoints

#### GET /:id
Returns the contents of the pasta with the given id. If the pasta is encrypted, the encrypted string will be returned, obviously without a password

#### Get /:id/:password
Returns the contents of the pasta with the given id, decrypted with the given password. If the pasta is not encrypted, the password is ignored.

#### POST /newPasta
Creates a new pasta with the given contents. Returns the id of the new pasta.

#### POST /newPastaSecure
Creates a new pasta with the given contents. Returns the id of the pasta, and the encryption key in JSON format. The encryption key is never stored on the server. So if you lose it, you're sol.
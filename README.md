# LightByte
## Fast and simple utility to compress and encrypt game assets

### About
When releasing a video game, you often have to compress game assets (e.g., textures, sounds)   
due to their large size and protect them from tampering.

Many game engines have their own solutions for this,   
but for people developing games using only frameworks or custom engines,   
it can be a somewhat challenging problem.   

For this, I created a lightweight program that compresses and encrypts game files!

This program compresses a folder with files into a `.pak` archive using the [zstd](https://en.wikipedia.org/wiki/Zstd)   
and encrypts it with [ChaCha20-Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305).

This program was mainly created for gamedev, but you can use it somewhere else.

### Building
```bash
git clone https://github.com/Ztry8/LightByte/
cd LightByte
cargo build
```

### Using
There are two operations:

#### lightbyte key
Generates a `key` file used for archive encryption.

The key is an array of 32 bytes. It is used to encrypt and decrypt your archive,  
so be careful with it and do not share this file with anyone.

The key is generated using the operating system's random number generator,  
making it cryptographically secure.

#### lightbyte compress <folder-name>
Creates and encrypts a `.pak` archive using the generated key file



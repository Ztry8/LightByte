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

This program was mainly created for game development, but it can be used for other purposes as well.

### Archive structure
1. **Files count** – 32-bit unsigned integer (`u32`)  
   Indicates the number of files in the archive.

2. **For each file:**
   - **File name length** – 16-bit unsigned integer (`u16`)  
     The length of the file name in bytes.
   - **File name** – array of 8-bit unsigned integers (`u8`)  
     The file name itself as a byte array.
   - **File content length** – 64-bit unsigned integer (`u64`)  
     The size of the file content in bytes.
   - **File content** – array of 8-bit unsigned integers (`u8`)  
     The actual file data.

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

#### lightbyte compress _folder-name_
Creates and encrypts a `.pak` archive from given folder using the generated key file.

### Example

#### Example of output
```bash
$ lightbyte key

LightByte v0.1.0 by Ztry8 (AslanD)
Utility to compress and encrypt game assets

Generating key...
Key generated successfully!

$ lightbyte compress textures

LightByte v0.1.0 by Ztry8 (AslanD)
Utility to compress and encrypt game assets

Compressing files...
Encrypting archive...
Archive created successfully!
```

#### Example of typical usage

```bash
# Generate a key file
$ lightbyte key
# => file `key` created

# Compress and encrypt the "textures" folder
$ lightbyte compress textures
# => file `textures.pak` created

# Compress and encrypt the "locations" folder
$ lightbyte compress locations
# => file `locations.pak` created

# Compress and encrypt the "dialogues" folder
$ lightbyte compress dialogues
# => file `dialogues.pak` created
```

#### Example of key file   
**(Do not use this key! Your key must remain private and no one should have access to it.)**
```
159
102
209
196
10
84
216
185
89
142
64
31
108
79
142
182
203
16
88
148
244
46
248
126
23
79
244
138
187
114
95
226
```


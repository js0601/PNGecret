# PNGecret

A command line tool to hide messages in PNG files


## Usage

#### Encode

Encode a secret message in test.png in a chunk with type TeST:

```
pngecret encode test.png TeST "message"
```


You can optionally encrypt the message using a passphrase:

```
pngecret encode test.png TeST "message" -e "passphrase"
```


You can provide an output file as well:

```
pngecret encode test.png TeST "message" -o testsecret.png
```


#### Decode

Decode the message in the chunk of type TeST:

```
pngecret decode test.png TeST
```


If the message was encrypted, you can decrypt it using the passphrase:

```
pngecret decode test.png TeST -d "passphrase"
```


#### Remove

Remove the secret message:

```
pngecret remove test.png TeST
```


#### Print

Print all chunks in a PNG file:

```
pngecret print test.png
```


## Resources

[PNGme](https://jrdngr.github.io/pngme_book/)

[PNG File Structure Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)

[Vigenère Cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)

# PNGecret

A command line tool to hide messages in PNG files


### Usage

Encode a secret message in test.png in a chunk with type TeST:

```
pngecret encode test.png TeST "secret message"
```

Decode the secret message:

```
pngecret decode test.png TeST
```

Remove the secret message:

```
pngecret remove test.png TeST
```

Print all chunks in a PNG file:

```
pngecret print test.png
```


### Resources

[PNGme](https://jrdngr.github.io/pngme_book/)

[PNG File Structure Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)

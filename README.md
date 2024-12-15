Rust carate implementing the communication protocol(https://novitus.pl/file/get/771) for the  Novitus Next device:
https://next.novitus.pl/en/next-online

Products can be uploaded from a csv file (ean,name,price,quantity,ptu):

```table
| ean | name              | price | quantity | ptu |
|-----|-------------------|-------|----------|-----|
| 21  | BW2 Poster 70x100 | 495   | 1        | A   |

```

Tool is also shipped with simple device emulator (to upload and download products).

The Novits Next device allows names up to 40 characters long. The tools use a Polish language dictionary, delivered with the crate to shorten names. 

I use this tool almost daily in a "production environment", but please be aware that there is always a risk of corrupting the product database.

For detailed usage instructions please refer to: [INSTRUCTIONS](INSTRUCTIONS.md)

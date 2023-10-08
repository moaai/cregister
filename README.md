Rust carate implementing the communication protocol(https://novitus.pl/file/get/771) for the  Novitus Next device:
https://next.novitus.pl/en/next-online

This tool was created to make it easier to update the product database.

Tool uploads products from a csv file (ean,name,price,quantity,ptu
) For example:

```table
| ean | name              | price | quantity | ptu |
|-----|-------------------|-------|----------|-----|
| 21  | BW2 Poster 70x100 | 495   | 1        | A   |


```

Downloaded products are also saved to csv file.

Tool is also shipped with simple device emulator (to upload and download products).

The Novits Next device allows names up to 40 characters long. The tools use a Polish language dictionary, delivered with the crate to shorten names. 

I use this tool almost every day in "production", but please be aware that there is always the possibility of breaking the products database, especially if other fields are required that are not supported by this tool.

For detailed usage instructions please refer to: [INSTRUCTIONS](INSTRUCTIONS.md)

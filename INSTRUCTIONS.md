1. Compilation

```bash
cargo build
```

After successful build two binaries are creates:
- clientapp
- device


2. Downloading products without device:
Crate is delivered with a simple device emulation (dowload and upload products)
- Start device:
```bash
RUST_LOG=info cargo r --bin device

...

2023-10-08T20:35:12Z INFO  cregister::device::cash_register] Server is listening on port 5001
```

Device emulator listens on the port 5001
- Run client (another terminal):
```bash
RUST_LOG=trace cargo r --bin clientapp -- -d 127.0.0.1 get products
```

The client downloads the products and saves them in the products.csv file. (if there are any problems, check if the device binary has access to the products.bin file)

3. Download products from device
- Configure your device to use the Tango over Ethernet protocol (enter the administration panel, data exchange). For Ethernet configuration, select the "server" option and set the port to, for example, 5001.
- Run the client application that provides the IP address of the device. The tool connects to port 5001 by default. If you set a different port, use the "-p" parameter.


4. Upload products from the products.csv file  to device.
```bash
RUST_LOG=trace cargo r --bin clientapp -- -d 127.0.0.1 send --file product.csv
```


Content of the product.csv file.
```
ean,name,price,quantity,ptu
1170,Serwetki papierowe 33x33 PAW Chru Rzod,1400.0,0,A
```

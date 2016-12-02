# allenap's TFTP offload server

This is an *experimental* TFTP offload server built using
[allenap's TFTP library for Rust](https://github.com/allenap/allenap-libtftp).

By *offload* I mean that this will do the work of receiving TFTP
requests and later transferring a file via TFTP, but another process —
in the first instance a [MAAS](https://maas.io/) `rackd` process with
small modifications — gets to decide which file to serve, and can even
generate that file on-the-fly.


## To do

 * Make the socket path configurable. At present it expects an
   `offload.sock` file in the working directory.

 * Document how to use this server with MAAS.

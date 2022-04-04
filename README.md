# ⚠️ Not supported

This crate is no longer supported and will not be updated. It was an experiment
and is unlikely to be useful to anyone in its present state. The code itself may
be a minor curiosity... but probably not. This is not the TFTP goodness you're
looking for. Sorry!

# allenap's TFTP offload server

This is an *experimental* TFTP offload server built using
[allenap's TFTP library for Rust](https://github.com/allenap/allenap-libtftp).

By *offload* I mean that this will do the work of receiving TFTP
requests and later transferring a file via TFTP, but another process —
in the first instance a [MAAS](https://maas.io/) `rackd` process with
small modifications — gets to decide which file to serve, and can even
generate that file on-the-fly.


## How to use this with MAAS on Ubuntu

This code is *experimental* and may break, so these instructions are
deliberately sparse: if you don't understand them as you read through
then you probably should *not* go any further.

Once the [*tftp-offload* merge proposal][tftp-offload-mp] has landed you
can switch to running MAAS from the [daily PPA][maas-daily-ppa]. In the
meantime you can build packages yourself by running `make package` in a
checkout of `lp:maas` and use `dpkg -i ../build-area/*.deb` to install
them.

Next you'll need [Rust][rust] >= 1.13 to build this code. The easiest
way is using [rustup][rustup]. This comes with *cargo* which you should
use to fetch, build and install the `allenap-tftp-offload` executable:

```console
$ cargo install allenap-tftp-offload
```

Create some *authbind* configuration:

```console
$ sudo touch /etc/authbind/byport/69
$ sudo chown maas /etc/authbind/byport/69
$ sudo chmod u+x /etc/authbind/byport/69
```

(An alternative here is to run `allenap-tftp-offload` as root.)

Start it up as *maas*:

```console
$ sudo -u maas authbind $(type -p allenap-tftp-offload) \
>   --socket /var/lib/maas/tftp-offload.socket
```

Then use MAAS as usual.


[tftp-offload-mp]: https://code.launchpad.net/~allenap/maas/tftp-offload/+merge/312146
[maas-daily-ppa]: https://launchpad.net/~maas-maintainers/+archive/ubuntu/dailybuilds
[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/


## To do

 * Make the socket path configurable. At present it expects an
   `offload.sock` file in the working directory.

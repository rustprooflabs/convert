# Convert:  Common conversion functions

Convert is a Postgres extension providing common conversion functions, such as meters to feet
or miles to kilometers.

The `convert` extension is built using the Rust [pgx framework](https://github.com/tcdi/pgx).

## Creating installer for your system

Currently no pre-packaged installers are available. The following steps walk through
creating a package on a typical Ubuntu based system with Postgres 14.
These steps assume cargo pgx is already installed.


The `fpm` step requires the `fpm` Ruby gem.

```bash
sudo apt install ruby-rubygems
sudo gem i fpm
```

> Timing note:  `cargo pgx package` takes ~ 2 minutes on my main dev machine.


```bash
cargo pgx package --pg-config /usr/lib/postgresql/14/bin/pg_config
cd target/release/convert-pg14/

find ./ -name "*.so" -exec strip {} \;
OUTFILE=convert.deb
rm ${OUTFILE} || true
fpm \
  -s dir \
  -t deb -n convert \
  -v 0.0.1 \
  --deb-no-default-config-files \
  -p ${OUTFILE} \
  -a amd64 \
  .

sudo dpkg -i --force-overwrite ./convert.deb
```


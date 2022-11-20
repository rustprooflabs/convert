# Convert:  Common spatial conversions

Instructions coming soon.

The extension is built on the Rust [pgx framework](https://github.com/tcdi/pgx).


```
cargo pgx package --pg-config /usr/lib/postgresql/14/bin/pg_config
cd target/release/convert-pg14/

find ./ -name "*.so" -exec strip {} \;
OUTNAME=convert
fpm \
  -s dir \
  -t deb -n convert \
  -v 0.0.1 \
  --deb-no-default-config-files \
  -p ${OUTNAME}.deb \
  -a amd64 \
  .

sudo dpkg -i --force-overwrite ./convert.deb
```


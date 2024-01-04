# redis-key

A command line utility that can read or write a value to a Redis instance based on a given key.

It can also connect via TLS, so it is useful for interacting with Redis instances running on Azure.
A typical usage would be to write and read values that are compressed.

This is how to read a gzipped value:

```bash
$ redis-key <redis-url> <key> | gunzip
```

This is how you can insert a value into Redis:

```bash
$ redis-key <redis-url> <key> <file-name>
```
where file-name is the name of the file containing the value to be written.

The Redis url format is the regular one, for instance for Azure:\
`rediss://:<password>@<hostname>:6380`


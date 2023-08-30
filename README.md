# meihao

A parallel path walker for file searching.

## Usage

Execute this in your shell:

```bash
meihao http://127.0.0.1/f .
```

Outputs:

```plain
{"hash":"ab5ecbc4868cdb0c17abdef7e91e13f4","url":"http://127.0.0.1/README.md"}
{"hash":"56f18d348631511976e6fb42153a7737","url":"http://127.0.0.1/Makefile"}
{"hash":"2fa6986977f1f037fa2a4100c234ef8e","url":"http://127.0.0.1/Cargo.toml"}
{"hash":"72395ed51d4d967b407cf17aac3cee4d","url":"http://127.0.0.1/Cargo.lock"}
```

Then follow [meilisearch - add docuemtns](https://www.meilisearch.com/docs/learn/getting_started/quick_start#add-documents) to import documents and you can search for files.

## License

MIT

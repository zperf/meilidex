# meilidex

A parallel path walker for file searching.

## Usage

Execute this in your shell:

```bash
meilidex --base-url http://127.0.0.1/f .
```

Outputs:

```plain
{"id":"2c0b7ae19eecf2a316c6d148057dcf42","path":"meilidex/Cargo.lock","mtime":"2024-02-21T11:47:26","file_size":"27.4 KiB","url":"http://127.0.0.1/Cargo.lock"}
{"id":"885a9c7b45f275a4f857179ab8cb54d6","path":"meilidex/README.md","mtime":"2024-02-21T11:35:16","file_size":"670 B","url":"http://127.0.0.1/README.md"}
{"id":"c59c472476a549caa54af36ddcc922f8","path":"meilidex/src/walker.rs","mtime":"2024-02-21T12:52:27","file_size":"2.9 KiB","url":"http://127.0.0.1/src/walker.rs"}
{"id":"c3088a9bf684d6e102a2feaedc7c41d2","path":"meilidex/Cargo.toml","mtime":"2024-02-21T11:47:24","file_size":"637 B","url":"http://127.0.0.1/Cargo.toml"}
{"id":"b3bf3d9ad6ebd0b28678b1a934458db9","path":"meilidex/src/main.rs","mtime":"2024-02-21T12:56:48","file_size":"1.2 KiB","url":"http://127.0.0.1/src/main.rs"}
```

Then follow [meilisearch - add documents](https://www.meilisearch.com/docs/learn/getting_started/quick_start#add-documents) to import documents, and you can search for files.

## License

MIT

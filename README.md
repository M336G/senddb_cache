# SendDB Cache
**A caching proxy for [SendDB](https://senddb.dev/) focusing on efficiently checking if levels have been sent or not**

## Prerequisites
- [Rust](https://rust-lang.org/) installed
- A device or server capable of running 24/7

## Running
**1.** [Download the repository manually](https://github.com/M336G/senddb_cache/archive/refs/heads/main.zip) or clone it:
```bash
git clone https://github.com/M336G/senddb_cache.git
cd senddb_cache
```

**2.** Create a `.env` file and fill it according to your needs using [`.env.example`](https://github.com/M336G/senddb_cache/blob/main/.env.example) as a template:
- Enter a **custom port** if you need to change the default one (8273)
- Specify **SendDB's endpoint URL** for accessing a level's data
- Set an **expiration** for how many minutes before temporarily cached not sent levels are able to be re-checked
- Set for **how long** sent levels will be cached by Cloudflare
- Set for **how long** not sent levels will be cached by Cloudflare

**3.** Start the instance with:
- `cargo run --release` for production
- `cargo run` for development/testing

**Once you've done all of this, you should have a running instance!**

## Usage
Once you've got your instance running, you may use these endpoints:

| Method | Endpoint      | Description                           |
|--------|---------------|---------------------------------------|
| `GET`  | `/`           | Check if the server's up or not       |
| `GET`  | `/stats`      | Get some statistics about the server  |
| `GET`  | `/level/<id>` | Check if a level has been sent or not |

Responses to the latter endpoint will look like this:
```json
{
    "error": null,
    "sent": true
}
```
- `error` will be `null` on success or a string describing the error
- `sent` will be `true` if the level has been sent, `false` if not, or `null` if an error occurred

## License
This project is licensed under the [MIT License](https://github.com/M336G/senddb_cache/blob/main/LICENSE).
use redis::Commands;

fn main() -> redis::RedisResult<()> {
    // Connect to the Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Set a key-value pair in Redis
    let _: () = con.set("my_key", 42)?;

    // Get the value back from Redis
    let my_value: isize = con.get("my_key")?;

    println!("Got value from Redis: {}", my_value);

    Ok(())
}

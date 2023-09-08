fn main() {
    let data = b"Hello, world!";

    println!("# xxhash-rs\n");
    println!("data : {:?}", data);
    println!();
    {
        let seed: u32 = 0x12345678;
        let digest = xxhash_rs::xxh_32::digest(data, seed);
        println!("XXH_32");
        println!("  seed : 0x{:08X}", seed);
        println!("digest : 0x{:08X}", digest);
    }
    println!();
    {
        let seed: u64 = 0x1020304050607080;
        let digest = xxhash_rs::xxh_64::digest(data, seed);
        println!("XXH_32");
        println!("  seed : 0x{:016X}", seed);
        println!("digest : 0x{:016X}", digest);
    }
}

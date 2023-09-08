import xxhash
import random

# Generate and return `n` random bytes in Rust syntax.
def rusty_randbytes(n):
	bytes = random.randbytes(n)
	return "[{}]".format(", ".join([f"0x{b:02X}" for b in bytes]))

# Converts bytes into a string representation (hex values, Rust syntax).
def rusty_bytes(bytes):
	return "[{}]".format(", ".join([f"0x{b:02X}" for b in bytes]))

# Generates a XXH 32-bit digest for bytes and seed.
def xxh32(bytes, seed):
	digest = xxhash.xxh32(bytes, seed)
	return "0x" + digest.hexdigest().upper()

# Generates a XXH 32-bit digest for bytes and seed.
def xxh64(bytes, seed):
	digest = xxhash.xxh64(bytes, seed)
	return "0x" + digest.hexdigest().upper()

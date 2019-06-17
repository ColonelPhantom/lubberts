// The types as defined in the Lua bytecode
// Recompile Lua code or change these if there is a conflict.
type Integer = u32;
type SizeT = u64; // should be usize, but this helps guarantee proper casting if non-native
type RawInstruction = u32;
type LuaNumber = f64;
const NUMBER_IS_INTEGRAL: bool = false; // Should be true if LuaNumber is integral (eg i32)

mod chunk;

fn main() {
    println!("Hello, world!");

    let data = std::fs::read("luac.out").expect("Unable to read file");

    println!("{:?}", chunk::Chunk::from_bytes(data.as_slice()));
}

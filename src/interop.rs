
#[link(name = "dotnet", kind = "static")]
extern "C" {
    pub fn dotnet_add(a: i32, b: i32) -> i32;
}
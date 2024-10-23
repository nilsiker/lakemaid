fn main() {
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-search=native={}", r"C:\Users\nilsi\.nuget\packages\runtime.win-x64.microsoft.dotnet.ilcompiler\8.0.10\sdk");
}

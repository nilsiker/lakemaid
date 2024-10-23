namespace dotnet;

using System.Runtime.InteropServices;

public static class NativeExports
{
    // This exposes the function to be callable from unmanaged code (like Rust)
    [UnmanagedCallersOnly(EntryPoint = "dotnet_add")]
    public static int Add(int a, int b)
    {
        return a + b;
    }
}

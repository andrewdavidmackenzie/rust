When I compiled with cargo, it compiled a .rlib file into target, but then I couldn't load this from ruby as a shared library.

Despite me adding this to the Cargo.toml file:
crate-type = ["dylib"]

To get it to work I had to compile using:

rustc src/lib.rs --crate-type=dylib

which compiles a liblib.dylib into the current directory.

I modified the embed.rb from the tutorial (book) to load it thus:

``
module Hello
  extend FFI::Library
  ffi_lib './liblib.dylib'
  attach_function :process, [], :void
end
``
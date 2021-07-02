# Intel MKL for Rust


## Build instructions on macOS

```bash
# workaround for searchpath of mkl-src
sudo mkdir -p /opt/intel/mkl/lib/
sudo ln -s /opt/intel/oneapi/mkl/latest/lib /opt/intel/mkl/lib/intel64

export PKG_CONFIG_PATH=/opt/intel/oneapi/mkl/latest/bin/pkgconfig/
export MKLROOT=/opt/intel/oneapi/mkl/latest/
export PKG_CONFIG_MKL_DYNAMIC_LP64_IOMP_MKLROOT=/opt/intel/oneapi/mkl/latest/
# verify it is found
pkg-config --libs mkl-dynamic-lp64-iomp

# to run the binary 
export DYLD_LIBRARY_PATH=/opt/intel/oneapi/mkl/latest/lib/
```

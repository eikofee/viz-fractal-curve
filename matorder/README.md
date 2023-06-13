# matorder
it's a tool to produce the **P** matrix to order matrices in a clustering approach.

The method used is **VAT** (https://ieeexplore.ieee.org/abstract/document/1007487).

## Dependencies
it's a C++ tool, and relies on CMake and ArrayFire:

### CMake
to install : `apt install cmake`

CMake will tell if something is missing (such as ArrayFire)

### ArrayFire
to install (Linux, taken from http://arrayfire.org/docs/installing.htm#Linux) :
  - Download AF here : https://arrayfire.com/download/
  - Run `./<ArrayFire .sh file> --include-subdir --prefix=/opt` (need root)
  - Run `export LD_LIBRARY_PATH=/opt/arrayfire/lib64` (each time or put in .bashrc or something)

If you install array fire elswhere you need to specify this variable (assume it is installed in `~/opt`):
`export ArrayFire_DIR=~/opt/arrayfire/`

### Building

#### Unix

```
mkdir build
cd build
cmake -DDEVICE=<CPU|CUDA|OPENCL> ..
make
```

#### Windows
- Install the Visual C++ Build Tools (https://aka.ms/buildtools, scroll down a bit)
- Launch a dev command prompt (usually at Start Menu > Visual Studio 20XX > x64/x86 native tools command prompt)
- Navigate to the project folder
```
mkdir build
cd build
cmake.exe -G "NMake Makefiles" -DCMAKE_CXX_STANDARD=11 -DCMAKE_BUILD_TYPE=Debug -DDEVICE=<CPU|CUDA|OPENCL> ..
nmake.exe
```

NMake may raise an error about not finding the `pthread.lib` file. A variant can be installed at https://sourceforge.net/projects/pthreads4w/files/.
Extract the archive, navigate to it using the same command prompt and run `nmake.exe install`. On the parent folder, a new folder `PTHREADS-BUILT` appeared.
Into this folder, takes the corresponding lib, rename it to `pthread.lib`, place it into the build folder and re-run `nmake.exe`.

Multiple DLLs are required to run the executables. You can either add the paths leading to those DLLs on the PATH environment variable or by copying the said
DLL files in the same directory as the executable (midly recommended because of the large size of CUDA DLLs).

### Usage
#### Computation
```
./order [debug] <mode> <input file> <output file> <matrix size>
```

   - `debug` : displays each steap and submatrix while generating **P**.
   - `mode` : either `read`, `load` or `convert`. `read` loads a matrix description file (see next section),
        while `load` loads an arrayfire binary file. `convert` is there to only transform a csv matrix into
        an arrayfire binary file.
   - `input file` : data to load according to the chosen `mode`. A csv file is needed for `read` and `convert`
        and an arrayfire binary file is needed for `load`. In the case of `debug`, this parameter is not used 
        (but still needed to the program, you can type anything in that case).
   - `output file` : For `read` and `load`, the path to save the ordered indexes to generate the **P** matrix.
        For `convert`, the path of the arrayfire binary file.
   - `matrix size` : size `n` of the `n` x `n` processed matrix.

#### Visualization
```
./order display <arrayfire matrix> <p matrix file> <matrix size>
./order image <arrayfire matrix> <p matrix file> <output name><.png|.jpg|.ppm> <matrix size>
```
   The arguments are similar to the previous section except this mode takes directly a file previously produced by the program, and the p matrix file must
   follow the same format as the one produced by this program.

   Matrix is being ordered according to the p matrix and displayed in a window (`display`) or saved on disk as an image (`image`).

### Input/Output Files

#### Matrix Description File
The file must be formatted as follows : `<X (rows)> <Y (column)> <Value (float)>` per line.
There is no need to input a full matrix (e.g. : `12 16 17.01` and later `16 12 17.01`).
`Value` must be the distance between the two concerned items, formatted as a `float` number,
using `.` as the decimal separator.

Both `read` and `convert` modes produce an arrayfire binary file to avoid re-reading the csv file. Loading
an arrayfire binary file is really faster than reading a csv file.

#### Result format
The produced file is a list in which each line corresponding to the P[i] value. As a reminder, the usage of
this matrix is `orderedD = D[all, P][P]` or `orderedD[x, y] = D[P[x], P[y]]` in a more conventional fashion.

### Additional notes
Windows' WSL1 cannot makes use of the GPU or CPU parallelization out-of-the-box. The program must be compiled
directly for the Windows platform for better performances.
However, a WSL process can be spawn multiple times on different CPU cores, so using the `convert` mode on multiple
order instances is viable.

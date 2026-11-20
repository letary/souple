@echo off
rem Builds the Yoga harness into crates\bench\yoga-build\ (git-ignored), two ways:
rem   msvc\Release\yoga-bench.exe   MSVC 19.29, /O2 /Ob2 /GL /LTCG
rem   clang\yoga-bench.exe          clang-cl 12 (VS 2019's LLVM), /O2 + ThinLTO, lld-link
rem Usage: crates\bench\yoga\build.bat  [YOGA_ROOT]   (default: ..\lecodes-next\engines\ui\libs\yoga)
setlocal
set HERE=%~dp0
set OUT=%HERE%..\yoga-build
set VS=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools
set YOGA=
if not "%~1"=="" set YOGA=-DYOGA_ROOT=%~1

cmake -S "%HERE%." -B "%OUT%\msvc" -G "Visual Studio 16 2019" -A x64 %YOGA% || exit /b 1
cmake --build "%OUT%\msvc" --config Release -- -m -v:minimal || exit /b 1

call "%VS%\VC\Auxiliary\Build\vcvars64.bat" >nul || exit /b 1
set LLVM=%VS%\VC\Tools\Llvm\x64\bin
cmake -S "%HERE%." -B "%OUT%\clang" -G Ninja -DCMAKE_BUILD_TYPE=Release ^
  "-DCMAKE_CXX_COMPILER=%LLVM%\clang-cl.exe" "-DCMAKE_LINKER=%LLVM%\lld-link.exe" "-DCMAKE_AR=%LLVM%\llvm-lib.exe" %YOGA% || exit /b 1
cmake --build "%OUT%\clang" || exit /b 1

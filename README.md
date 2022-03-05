This project is a small reproduction of two issues.

First is that maturin is looking in the wrong directory for the header.h when the crate is part of a workspace

The second is that the header.h isn't included in the source distribution

To use the compiled wheel and see it work, in a venv run:

`cargo xtask && pip install --force-reinstall target/wheels/pythonsdk-0.1.0-py3-none-linux_x86_64.whl && python test.py`

and you will see:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/xtask`
Building rust packages
Generating headers
Prepping the header for cffi
Building python wheels
Processing ./target/wheels/pythonsdk-0.1.0-py3-none-linux_x86_64.whl
Collecting cffi
  Using cached cffi-1.15.0-cp38-cp38-manylinux_2_12_x86_64.manylinux2010_x86_64.whl (446 kB)
Collecting pycparser
  Using cached pycparser-2.21-py2.py3-none-any.whl (118 kB)
Installing collected packages: pycparser, cffi, pythonsdk
  Attempting uninstall: pycparser
    Found existing installation: pycparser 2.21
    Uninstalling pycparser-2.21:
      Successfully uninstalled pycparser-2.21
  Attempting uninstall: cffi
    Found existing installation: cffi 1.15.0
    Uninstalling cffi-1.15.0:
      Successfully uninstalled cffi-1.15.0
  Attempting uninstall: pythonsdk
    Found existing installation: pythonsdk 0.1.0
    Uninstalling pythonsdk-0.1.0:
      Successfully uninstalled pythonsdk-0.1.0
Successfully installed cffi-1.15.0 pycparser-2.21 pythonsdk-0.1.0
Hello world!
```

To see if fail with a source distribution, run:
`cargo xtask && pip install --force-reinstall target/wheels/pythonsdk-0.1.0.tar.gz  && python test.py`
and you will see
```
Building rust packages
Generating headers
Prepping the header for cffi
Building python wheels
Processing ./target/wheels/pythonsdk-0.1.0.tar.gz
  Installing build dependencies ... done
  Getting requirements to build wheel ... done
  Installing backend dependencies ... done
  Preparing metadata (pyproject.toml) ... done
Collecting cffi
  Using cached cffi-1.15.0-cp38-cp38-manylinux_2_12_x86_64.manylinux2010_x86_64.whl (446 kB)
Collecting pycparser
  Using cached pycparser-2.21-py2.py3-none-any.whl (118 kB)
Building wheels for collected packages: pythonsdk
  Building wheel for pythonsdk (pyproject.toml) ... done
  Created wheel for pythonsdk: filename=pythonsdk-0.1.0-py3-none-linux_x86_64.whl size=871781 sha256=5a75d240fb9d6594d474236a13dbeaca03b24e7efbe39a1639cb9d55a7640a77
  Stored in directory: /home/stusmall/.cache/pip/wheels/1b/57/9e/93c3823abd0c72f2a744d2be5c83ce25fd82ca7717f6652f50
Successfully built pythonsdk
Installing collected packages: pycparser, cffi, pythonsdk
  Attempting uninstall: pycparser
    Found existing installation: pycparser 2.21
    Uninstalling pycparser-2.21:
      Successfully uninstalled pycparser-2.21
  Attempting uninstall: cffi
    Found existing installation: cffi 1.15.0
    Uninstalling cffi-1.15.0:
      Successfully uninstalled cffi-1.15.0
  Attempting uninstall: pythonsdk
    Found existing installation: pythonsdk 0.1.0
    Uninstalling pythonsdk-0.1.0:
      Successfully uninstalled pythonsdk-0.1.0
Successfully installed cffi-1.15.0 pycparser-2.21 pythonsdk-0.1.0
Traceback (most recent call last):
  File "test.py", line 1, in <module>
    import pythonsdk
  File "/home/stusmall/Workspace/portscanner/sdks/python/venv/lib/python3.8/site-packages/pythonsdk/__init__.py", line 2, in <module>
    lib.hello_world()
AttributeError: cffi library '/home/stusmall/Workspace/portscanner/sdks/python/venv/lib/python3.8/site-packages/pythonsdk/pythonsdk/native.so' has no function, constant or global variable named 'hello_world'
```

This is because the custom header file isn't included and maturin fails back to cbindgen which files to find the methods
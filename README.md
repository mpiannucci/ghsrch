# ghsrch

Search github programmatically.

## Developing

Create a new virtual environment and install the requirements:

```bash
virtualenv -p python3 env
source env/bin/activate
pip install maturin
```

#### Rust Library

Build the rust library:

```bash
cargo build
```

#### Python Library

Build the python library:

```bash
maturin develop
```

Then you can verify that the library is working:

```python
import ghsrch

# TODO: Show the usage
```

Or you can run the example command line tool:

```bash
ghsrchr --help
```

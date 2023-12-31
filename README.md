# ghsrch

Search github programmatically.

## Developing

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Python](https://www.python.org/downloads/)
- [virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)

You will also need a a github personal access token. You can create one [with the instructions here](https://docs.github.com/en/rest/quickstart?apiVersion=2022-11-28&tool=curl). Save this token and do not save it in a public place.

### Building and Running

Create a new virtual environment and install the requirements:

```bash
virtualenv -p python3 env
source env/bin/activate
pip install maturin
```

#### Rust Library

Test the rust library:

```bash
cargo test
```

#### Python Library

Build the python library:

```bash
maturin develop
```

Then you can verify that the library is importable:

```python
import ghsrch
```

Or you can run the example command line tool, after setting your github token as an environment variable:

```bash
export GITHUB_TOKEN=<your token>

ghsrchr --help

# example search for rust code using the zarr keyword
python example/ghsrchr.py code --language=rust  zarr

# example search commits 
python example/ghsrchr.py commits --repo=pydata/xarray cache
```

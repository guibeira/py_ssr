from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="py_ssr",
    version="0.1.0",
    rust_extensions=[RustExtension("py_ssr", "Cargo.toml", binding="pyo3")],
    packages=["py_ssr"],
    zip_safe=False,
)

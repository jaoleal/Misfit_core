from setuptools import setup, find_packages

setup(
    name='misfit-core',
    version='0.0.1',
    install_requires=[],
    packages=find_packages(),
    entry_points={
        'console_scripts': [
            'misfit-core=src.misfit_core:main',
        ],
    },
    author='TheMhv, Joaozinho',
    author_email='',
    description='A tool for create food for test',
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    python_requires='>=3.8',  # Minimum Python version
)
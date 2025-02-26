About

Eland is a Python Elasticsearch client for exploring and analyzing data in Elasticsearch with a familiar Pandas-compatible API.

Where possible the package uses existing Python APIs and data structures to make it easy to switch between numpy, pandas, or scikit-learn to their Elasticsearch powered equivalents. In general, the data resides in Elasticsearch and not in memory, which allows Eland to access large datasets stored in Elasticsearch.

Eland also provides tools to upload trained machine learning models from common libraries like scikit-learn, XGBoost, and LightGBM into Elasticsearch.
Getting Started

Eland can be installed from PyPI with Pip:

$ python -m pip install eland

If using Eland to upload NLP models to Elasticsearch install the PyTorch extras:

$ python -m pip install 'eland[pytorch]'

Eland can also be installed from Conda Forge with Conda:

$ conda install -c conda-forge eland

Compatibility

    Supports Python 3.9, 3.10, 3.11, 3.12 and Pandas 1.5
    Supports Elasticsearch 8+ clusters, recommended 8.16 or later for all features to work. If you are using the NLP with PyTorch feature make sure your Eland minor version matches the minor version of your Elasticsearch cluster. For all other features it is sufficient for the major versions to match.
    You need to install the appropriate version of PyTorch to import an NLP model. Run python -m pip install 'eland[pytorch]' to install that version.

Prerequisites

Users installing Eland on Debian-based distributions may need to install prerequisite packages for the transitive dependencies of Eland:

$ sudo apt-get install -y \
  build-essential pkg-config cmake \
  python3-dev libzip-dev libjpeg-dev

Note that other distributions such as CentOS, RedHat, Arch, etc. may require using a different package manager and specifying different package names.
Docker

If you want to use Eland without installing it just to run the available scripts, use the Docker image. It can be used interactively:

$ docker run -it --rm --network host docker.elastic.co/eland/eland

Running installed scripts is also possible without an interactive shell, e.g.:

$ docker run -it --rm --network host \
    docker.elastic.co/eland/eland \
    eland_import_hub_model \
      --url http://host.docker.internal:9200/ \
      --hub-model-id elastic/distilbert-base-cased-finetuned-conll03-english \
      --task-type ner
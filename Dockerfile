# Use a specific Ubuntu LTS version as the base
FROM ubuntu:22.04

# Set frontend to noninteractive to avoid prompts during package installs
ENV DEBIAN_FRONTEND=noninteractive

# --- Install Base Dependencies (Build tools, curl, GPG, Git) ---
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    software-properties-common \
    gnupg \
    gpg-agent \
    curl \
    wget \
    build-essential \
    ca-certificates \
    git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# --- Add PPA and install Python 3.12 ---
RUN add-apt-repository ppa:deadsnakes/ppa && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    python3.12 \
    python3.12-dev \
    python3.12-venv && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# --- Install pip using get-pip.py for Python 3.12 ---
RUN curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py && \
    python3.12 get-pip.py && \
    rm get-pip.py

# --- Set Python 3.12 as default ---
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.12 1 && \
    update-alternatives --install /usr/bin/pip3 pip3 /usr/local/bin/pip 1

# --- Install Rustup and the specific Rust toolchain ---
ENV RUSTUP_HOME=/root/.rustup
ENV CARGO_HOME=/root/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y

RUN rustup install nightly-2022-08-08 && \
    rustup component add rust-src rustc-dev llvm-tools-preview --toolchain nightly-2022-08-08 && \
    rustup default nightly-2022-08-08

# --- Set Rust Environment Variables ---
ENV RUST_TOOLCHAIN_LIB_PATH="${RUSTUP_HOME}/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib"
ENV RUSTFLAGS="-L ${RUST_TOOLCHAIN_LIB_PATH}"
ENV LD_LIBRARY_PATH="${RUST_TOOLCHAIN_LIB_PATH}:${LD_LIBRARY_PATH:-}"

# --- Setup placeholder application directory ---
# The actual code will be mounted here at runtime
WORKDIR /app

# --- Install Build Dependencies ---
# Copy ONLY the necessary files for dependency installation and tool building
# Assumes Dockerfile is in the SAME directory as 'c2saferrust/'
COPY ./requirements.txt /app/
COPY ./slicer /app/slicer/
COPY ./metrics /app/metrics/

# Install Python Dependencies from the copied requirements file
RUN pip3 install --no-cache-dir -r /app/requirements.txt
# Install potentially missing optional dependencies (like google)
RUN pip3 install --no-cache-dir google-generativeai pydot

# Build and Install Rust Tools (Slicer and Metrics) using the copied sources
RUN cd /app/slicer && \
    cargo install --debug --locked --path . --force && \
    cd /app # Go back to /app

RUN cd /app/metrics && \
    cargo install --debug --locked --path . --force && \
    cd /app # Go back to /app

# --- Clean up copied source code ---
# Remove the source code used for building tools, as the live code
# will be mounted from the host at runtime. This keeps the image clean.
RUN rm -rf /app/requirements.txt /app/slicer /app/metrics

# --- API Key ---
# No API key handling during build. Will be passed at runtime.

# --- Verification ---
RUN echo "Python version: $(python3 --version)" && \
    echo "Pip version: $(pip3 --version)" && \
    echo "Rust version: $(rustc --version)" && \
    echo "Cargo version: $(cargo --version)" && \
    echo "Slicer installed: $(command -v slicer || echo 'Not found in build path')" && \
    echo "Metrics installed: $(command -v metrics || echo 'Not found in build path')"
    # Note: Slicer/Metrics are installed to $CARGO_HOME/bin, which is in PATH

CMD ["/bin/bash"]
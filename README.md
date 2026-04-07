# RSLWE: Relatively Simple Learning With Errors

[](https://www.rust-lang.org)
[](https://opensource.org/licenses/MIT)

**RSLWE** is a high-performance, pedagogical, and strictly-typed implementation of the **Learning With Errors (LWE)** cryptosystem in Rust. This project is designed for researchers and developers to analyze the core mechanics of Lattice-Based Cryptography through modular arithmetic, discrete distributions, and deep performance profiling.

---

## 🚀 Key Features

- **Core LWE Primitive**: Full implementation of KeyGen, Encryption, and Decryption.
- **Discrete Gaussian Sampling**: High-precision noise generation using a Cumulative Distribution Table (CDT) and `libm`.
- **Flexible Distribution API**: Extensible `Sampler` trait supporting Uniform, Binary, and Gaussian distributions.
- **Elite Benchmarking Suite**: Integrated with **Criterion** (statistical time), **Iai-Callgrind** (instruction counts/cache hits), and **Divan**.
- **Advanced Analytics**: Custom performance dashboard and flamegraph-ready profiling.

---

## 📊 Performance & Analytics

We treat performance as a first-class citizen. You can find detailed reports in the `./docs` directory.

### [Interactive Performance Dashboard](./docs/index.html)

Our custom dashboard aggregates data from multiple sources:

- **Instruction Profiling**: Exact CPU instruction counts via Valgrind.
- **Cache Analysis**: L1/LL cache hit rates to identify memory bottlenecks.
- **Micro-benchmarking**: Nanosecond-precision timing for hot paths.

### [Flamegraph Analysis](./docs/flamegraph.svg)

Visualizing the "hot paths" of the encryption and decryption cycles.
_(Note: Ensure you generate the flamegraph using `cargo flamegraph` to update this file)._

---

## 🛠 Mathematical Structure

The system operates on the following LWE parameters:

- **$n$**: Secret key dimension.
- **$m$**: Number of samples (rows in public matrix $A$).
- **$q$**: Ciphertext modulus.

### Encryption Logic

Given a public key $(A, b)$ and a bit $m \in \{0, 1\}$:

1.  Sample a random small vector $r \in \{0, 1\}^m$.
2.  Compute $u = A^T r \pmod q$.
3.  Compute $v = b^T r + m \cdot \lfloor q/2 \rfloor \pmod q$.
4.  Result is the ciphertext $(u, v)$.

---

## 💻 Usage

### Quick Start

To run the demo where Alice and Bob exchange an encrypted string:

```bash
cargo run --release
```

### 🧪 Advanced Benchmarking & Resource Isolation

The project uses a sophisticated benchmarking stack to ensure "clean room" results. By strictly bounding CPU and Memory, we eliminate OS jitter and frequency scaling noise.

#### 1. Resource Isolation (`isolate.sh`)

The `isolate.sh` utility leverages **Linux cgroups v2** (via `systemd-run`) to strictly bound the benchmarking process. This prevents results from being skewed by background tasks.

```bash
# Example: Run benchmarks with 50% CPU quota, 512MB RAM, pinned to Core 2
./isolate.sh -c 50000 -m 512M -p 2 -- cargo bench
```

**Key Isolation Features:**

- **CPU Pinning**: Locks the process to a specific core (default: Core 2) to minimize cache migrations and context switching.
- **Quota Enforcement**: Strict CPU time (μs) and Memory limits (default: 1GB).
- **Governor Control**: Automatically switches the CPU to `performance` mode during execution and restores `powersave` on exit.

#### 2. Automated Data Collection (`collect_data.sh`)

The `collect_data.sh` script automates the entire performance pipeline. It triggers the isolated benchmarks and regenerates the `./docs/data` directory with fresh metrics.

```bash
# Regenerate all data with custom iterations (-k: keygen, -e: encrypt, -d: decrypt)
# Note : custom iterations are not working for now.
# Default values : k = 100, e = 1000, d = 1000
./collect_data.sh -k 100 -e 1000 -d 1000
```

**Pipeline Workflow:**

1.  **Environment Capture**: Generates `env.json` with CPU model, frequency limits, and timestamp.
2.  **Iai-Callgrind**: Performs hardware-agnostic instruction counting and cache profiling.
3.  **Criterion**: Conducts statistical time analysis and generates distribution plots.
4.  **Ownership Management**: Automatically fixes `target/` permissions after `sudo` operations to ensure data remains accessible.

---

### 📊 Manual Benchmarking

Alternatively, you can run specific tools directly (though results may be less stable without the isolation script):

```bash
# Statistical time benchmarks (Criterion)
cargo bench --bench criterion_lwe

# Hardware-agnostic instruction counting (Iai-Callgrind)
cargo bench --bench iai_lwe

# Fast & versatile micro-benchmarking (Divan)
cargo bench --bench divan_lwe
```

### 📈 Analytics Output

After running the collector, visualize the results in:

- **[Interactive Performance Dashboard](./docs/index.html)**: A custom view merging instructions and timing.
- **[Criterion Reports](./docs/reports/criterion/index.html)**: Full statistical breakdown and overhead analysis.

## 📂 Project Architecture

```text
rslwe/
├── src/
│   ├── lwe/          # Core Cryptography (KeyGen, Enc, Dec)
│   ├── utils/        # Math primitives (Matrix, Samplers)
│   └── lib.rs        # Injection-based configuration
├── benches/          # Multi-tool benchmarking suite
└── docs/             # Visual reports & Performance Dashboard
```

---

## 🚀 Roadmap & Future Work

RSLWE is currently in **Beta**. It is a platform for research and learning, with the following improvements planned:

- [ ] **Robustness & Security**:
    - Implement **Constant-Time** operations to mitigate side-channel attacks.
    - Develop a dedicated **Security Analysis Suite** (automated tests for noise-to-error ratios and lattice reduction hardness).
- [ ] **System Enhancements**:
    - Full **`no_std`** support for embedded systems and WASM.
    - Integrated **CSPRNG** (Cryptographically Secure Pseudo-Random Number Generator) to remove manual seeding.
- [ ] **Optimization**:
    - SIMD/AVX2 acceleration for matrix operations.
    - NTT (Number Theoretic Transform) integration for faster polynomial-like multiplications.
- [ ] **Testing**: Increase code coverage with property-based testing (Proptest).

---

## 🧪 Security Note

**WARNING**: This is a research and educational implementation. It has not undergone a professional security audit. It is **not** currently hardened against side-channel attacks and should not be used for production data.

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

---

_Built with 🦀 By Luis._

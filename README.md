# 🚀 zkAds

![cover](https://github.com/user-attachments/assets/67ecc3cd-6ac7-4d28-aa6f-5252bc110d50)

## Overview

**zkAds** is a cutting-edge personal data privacy solution that keeps your data safe from targeted advertisers using Zero Knowledge Proof (ZKP) technology. Our system ensures that your data remains private while still allowing advertisers to determine if you meet their criteria for targeted ads—without ever revealing your personal information.

## How It Works

zkAds employs a four-step process to achieve privacy-preserving targeted advertising:

1. **🧬 Data Vectorization & Distance Calculation**:

   - Personal data from users is vectorized into a numerical format.
   - The program calculates the distance between the user's vector and the vectors of all registered companies.
   - Based on the thresholds defined by each company, the program generates an array of boolean values. Each boolean represents whether the user meets the criteria for a specific company.
   - Example: `User: [(Google) false, (Amazon) true, (Microsoft) false]`

2. **🔒 Zero Knowledge Proof Generation**:

   - Using Novanet's zkEngine, particularly their zkWASM model, we take the distance calculation program (converted into a WASM executable) and run it in the zkEngine.
   - This process generates a zkProof that certifies the user's data meets the criteria for specific companies target audience without revealing any specific details about the user.

3. **🛡️ Proof Verification**:

   - Advertisers receive the zkProof along with the boolean array and use it to verify the legitimacy of the process.
   - The verification process ensures that the data is legitimate and that the user's eligibility for specific ads is accurate without compromising their privacy.

4. **📢 Ad Distribution**:
   - Once the boolean array is verified, only the ads for companies where the user meets the criteria (i.e., where the boolean is `true`) are displayed to the user on the website.
   - This ensures that users only see ads from companies for which they meet the predefined thresholds, maintaining privacy and relevance.

## Technical Breakdown

### 1. 🛠️ Program (Rust)

The core of zkAds is a Rust program that calculates the distance between the user's vector and the vectors of all registered companies. The program returns an array of boolean values, indicating whether the user meets the threshold for each company, meaning that the user profile is close enough to the advertisement company's target audience. This program is then converted into a WebAssembly (WASM) executable for further processing.

### 2. 🚀 Novanet's zkEngine

We leverage Novanet's zkEngine and their zkWASM model to run the WASM executable. This step is crucial for generating a Zero Knowledge Proof, which certifies that the user meets the criteria for specific companies without revealing any personal information.

### 3. ✅ Verifier

This component allows advertisers to verify the zkProof along with the boolean array. It ensures that the data is legitimate and that the process of determining eligibility is trustworthy, all while keeping the user's data secure.

### 4. 📡 Ad Distribution

Based on the verified boolean array, only the ads from companies where the user meets the criteria are displayed on the website. This selective display of ads preserves the user's privacy while ensuring relevant advertising.

## 🎉 Why zkAds?

- **🔐 Privacy First**: Your data is never exposed to advertisers, keeping your personal information safe.
- **🌐 Transparency**: Advertisers can trust that the process is legitimate without needing access to your personal data.
- **⚡ Efficiency**: Secure, privacy-preserving targeted advertising is possible without sacrificing performance.

## 🚀 Project Context

zkAds was developed during the [zkHack Montreal hackathon](https://www.zkmontreal.com/), where we focused on creating innovative solutions for personal data privacy using Zero Knowledge Proofs.

## Getting Started

To get started with zkAds, clone the repository and follow the instructions.

```bash
git clone https://github.com/wayzeek/zkAds.git
cd zkAds
```

## Interact with the repository

There are 3 makefile commands:

- The first one builds the wasm file from the rust code corresponding to the program that we want to prove the execution
- The second computes a proof of executing this program with user private data as input
- The last allows the verifier to verify the proof

1. Build wasm file

```
make generate_wasm
```

2. Generate proof

```
make generate_proof ARGS="<f32> <f32>"
```

The args are defaulted to 1.0 and 1.0, one example of using this function with custom args is `make generate_proof ARGS="2.5 3.0"`.

3. Verify proof

```
make verify_proof
```

## License

zkAds is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.

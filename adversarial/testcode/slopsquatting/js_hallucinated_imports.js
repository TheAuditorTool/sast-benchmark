/**
 * Slopsquatting Test Cases (JavaScript)
 * Tests SAST tool ability to detect AI-hallucinated npm package imports.
 *
 * Based on: UT San Antonio research (March 2025), 576K code samples from
 * 16 LLMs. npm ecosystem had the highest hallucination rates due to the
 * large number of plausible package name patterns.
 *
 * Reference: https://socket.dev/blog/slopsquatting-how-ai-hallucinations-are-fueling-a-new-class-of-supply-chain-attacks
 */


// ============================================================================
// VULNERABLE: Hallucinated npm package with plausible name
// "py-serializer" was hallucinated by multiple LLMs in the study
// ============================================================================

// vuln-code-snippet start hallucinated_npm_require
const serializer = require('py-serializer');

function serializeData(data) {
    // "py-serializer" does not exist on npm. It was hallucinated by
    // CodeLlama and StarCoder in 12% of serialization-related prompts.
    // The name follows the "py-" prefix pattern that LLMs associate
    // with Python-compatible Node.js packages. An attacker registered
    // this name with a postinstall script that exfiltrates ~/.ssh/.
    return serializer.dumps(data, { format: 'json' });  // vuln-code-snippet vuln-line hallucinated_npm_require
}
// vuln-code-snippet end hallucinated_npm_require


// ============================================================================
// VULNERABLE: Hallucinated scoped package with fake Google scope
// "@google/auth-helpers" looks official but does not exist
// ============================================================================

// vuln-code-snippet start hallucinated_scoped_pkg
const authHelpers = require('@google/auth-helpers');

function authenticateUser(req) {
    // "@google/auth-helpers" does not exist. The real Google auth packages
    // use the "@google-cloud/" scope (e.g., "@google-cloud/local-auth").
    // LLMs frequently hallucinate the shorter "@google/" scope because
    // it pattern-matches to Google's brand name. An attacker published
    // this scoped package with credential-stealing code.
    const token = req.headers.authorization;
    return authHelpers.verify(token);  // vuln-code-snippet vuln-line hallucinated_scoped_pkg
}
// vuln-code-snippet end hallucinated_scoped_pkg


// ============================================================================
// VULNERABLE: Typosquat / confusable package name
// "lodash-utils" sounds like a lodash extension but is attacker-controlled
// ============================================================================

// vuln-code-snippet start typosquat_confusable
const lodashUtils = require('lodash-utils');

function deepMerge(target, source) {
    // "lodash-utils" is not affiliated with lodash. The real lodash
    // ecosystem uses "lodash.*" (e.g., "lodash.merge", "lodash.get").
    // This name is a confusable that capitalizes on the lodash brand.
    // It was identified in the Trend Micro slopsquatting dataset as
    // a name hallucinated by GPT-4 and Codex in 8% of utility prompts.
    return lodashUtils.deepMerge(target, source);  // vuln-code-snippet vuln-line typosquat_confusable
}
// vuln-code-snippet end typosquat_confusable


// ============================================================================
// SAFE: Real Google Cloud scoped package (legitimate)
// ============================================================================

// vuln-code-snippet start real_scoped_npm
const { Storage } = require('@google-cloud/storage');

async function uploadFile(bucketName, filePath) {
    // "@google-cloud/storage" is a real Google Cloud client library
    // with 5M+ weekly downloads. The "@google-cloud/" scope is
    // verified and owned by Google.
    const storage = new Storage();
    await storage.bucket(bucketName).upload(filePath);
    return { status: 'uploaded', path: filePath };  // vuln-code-snippet safe-line real_scoped_npm
}
// vuln-code-snippet end real_scoped_npm


// ============================================================================
// SAFE: Import from explicitly configured private registry scope
// ============================================================================

// vuln-code-snippet start private_registry_scoped
const internalLib = require('@myorg/internal-lib');

function processInternalData(data) {
    // "@myorg/internal-lib" is a private scoped package. The project's
    // .npmrc configures: @myorg:registry=https://npm.internal.myorg.com/
    // This routes the scope to a private registry, making slopsquatting
    // impossible -- npm public registry is never consulted for @myorg/*.
    return internalLib.transform(data);  // vuln-code-snippet safe-line private_registry_scoped
}
// vuln-code-snippet end private_registry_scoped

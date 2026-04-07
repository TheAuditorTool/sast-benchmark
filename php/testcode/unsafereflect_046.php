<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_plugin_signature_verify
function unsafereflect046(BenchmarkRequest $req): BenchmarkResponse {
    $plugin = $req->param('plugin');
    $sig = $req->header('X-Plugin-Sig');
    if (!openssl_verify($plugin, base64_decode($sig), file_get_contents('/etc/app/plugin.pub'), OPENSSL_ALGO_SHA256)) { // vuln-code-snippet safe-line php_reflect_plugin_signature_verify
        return BenchmarkResponse::badRequest('invalid sig');
    }
    return BenchmarkResponse::ok('verified');
}
// vuln-code-snippet end php_reflect_plugin_signature_verify

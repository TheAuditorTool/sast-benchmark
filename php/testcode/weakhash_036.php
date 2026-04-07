<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sodium_pwhash
function weakhash036(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = sodium_crypto_pwhash_str( // vuln-code-snippet safe-line php_weakhash_sodium_pwhash
        $pass,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE
    );
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sodium_pwhash

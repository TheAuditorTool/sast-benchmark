<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_magic_hash_sha1
function typejuggling020(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('hash');
    if (sha1($input) == '0e89100756225895e68e7ba9189a28f18fc8bcc5') { // vuln-code-snippet vuln-line php_tj_magic_hash_sha1 // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_magic_hash_sha1

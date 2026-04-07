<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_magic_hash_md5
function typejuggling019(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('hash');
    if (md5($input) == '0e462097431906509019562988736854') { // vuln-code-snippet vuln-line php_tj_magic_hash_md5 // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_magic_hash_md5

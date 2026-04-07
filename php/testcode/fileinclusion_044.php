<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_hash_lookup
function fileinclusion044(BenchmarkRequest $req): BenchmarkResponse {
    $templateMap = [md5('home') => 'home.php', md5('about') => 'about.php'];
    $key  = $req->param('key');
    $file = $templateMap[md5($key)] ?? 'default.php'; // vuln-code-snippet safe-line php_fi_hash_lookup
    include __DIR__ . '/pages/' . $file;
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_hash_lookup

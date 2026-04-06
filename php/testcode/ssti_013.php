<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_mustache_file
function ssti013(BenchmarkRequest $req): BenchmarkResponse {
    $engine = new Mustache_Engine([
        'loader' => new Mustache_Loader_FilesystemLoader(__DIR__ . '/templates'),
    ]);
    $output = $engine->render('greeting', ['name' => $req->param('name')]); // vuln-code-snippet safe-line php_ssti_mustache_file
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_mustache_file

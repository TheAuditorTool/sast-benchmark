<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_mustache_file_based
function ssti038(BenchmarkRequest $req): BenchmarkResponse {
    $m = new Mustache_Engine(['loader' => new Mustache_Loader_FilesystemLoader('/views')]);
    $tmpl = $m->loadTemplate('user_card'); // vuln-code-snippet safe-line php_ssti_mustache_file_based
    $html = $tmpl->render(['name' => $req->param('name')]);
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_mustache_file_based

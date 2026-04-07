<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_registry_map
function fileinclusion038(BenchmarkRequest $req): BenchmarkResponse {
    $map = ['list' => 'list.php', 'detail' => 'detail.php'];
    $key = $req->param('key');
    $tpl = $map[$key] ?? 'default.php'; // vuln-code-snippet safe-line php_fi_registry_map
    include __DIR__ . '/pages/' . $tpl;
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_registry_map

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_dto_named_args
function extract045(BenchmarkRequest $req): BenchmarkResponse {
    $p = $req->post('prefs') ?? [];
    $prefs = new UserPrefs(lang: $p['lang'] ?? 'en', theme: $p['theme'] ?? 'default'); // vuln-code-snippet safe-line php_extract_dto_named_args
    return BenchmarkResponse::ok("lang={$prefs->lang}");
}
// vuln-code-snippet end php_extract_dto_named_args

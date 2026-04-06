<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_compact_roundtrip
function extract010(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $role = $req->post('role');
    $packed = compact('name', 'role');
    $is_admin = false;
    extract($packed); // vuln-code-snippet vuln-line php_extract_compact_roundtrip
    return BenchmarkResponse::json(['name' => $name, 'role' => $role, 'admin' => $is_admin]);
}
// vuln-code-snippet end php_extract_compact_roundtrip

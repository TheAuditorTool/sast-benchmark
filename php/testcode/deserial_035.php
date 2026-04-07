<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_allowed_classes_false
function deserial035(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $data = unserialize($raw, ['allowed_classes' => false]); // vuln-code-snippet safe-line php_deser_allowed_classes_false
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('expected array');
    }
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_allowed_classes_false

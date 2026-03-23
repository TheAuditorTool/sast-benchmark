<?php
require_once __DIR__ . '/shared.php';

class SafeDTO {
    public string $name;
    public int $age;
}

// vuln-code-snippet start php_deser_allowed_classes
function deserial_allowed_classes(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data, ['allowed_classes' => ['SafeDTO']]); // vuln-code-snippet safe-line php_deser_allowed_classes
    return BenchmarkResponse::json(['name' => $obj->name ?? 'unknown']);
}
// vuln-code-snippet end php_deser_allowed_classes

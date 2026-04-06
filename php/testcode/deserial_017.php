<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_igbinary_typecheck
class SafeDTO017 {
    public string $name;
    public string $email;
}

function deserial017(BenchmarkRequest $req): BenchmarkResponse {
    $raw = base64_decode($req->post('payload'));
    $obj = igbinary_unserialize($raw);
    if (!($obj instanceof SafeDTO017)) { // vuln-code-snippet safe-line php_deser_igbinary_typecheck
        return BenchmarkResponse::badRequest('Invalid payload type');
    }
    return BenchmarkResponse::json(['name' => $obj->name, 'email' => $obj->email]);
}
// vuln-code-snippet end php_deser_igbinary_typecheck

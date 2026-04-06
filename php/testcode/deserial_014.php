<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_destruct_gadget
class CacheFile014 {
    public string $path;
    public function __destruct() {
        if (file_exists($this->path)) {
            unlink($this->path);
        }
    }
}

function deserial014(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('session');
    $obj = unserialize($data); // vuln-code-snippet vuln-line php_deser_destruct_gadget
    return BenchmarkResponse::ok('Session loaded');
}
// vuln-code-snippet end php_deser_destruct_gadget

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_tostring_file_read
class FileReader030 {
    public string $file = '';
    public function __toString(): string {
        return file_get_contents($this->file);
    }
}

function deserial030(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data); // vuln-code-snippet vuln-line php_deser_tostring_file_read
    $output = (string) $obj;
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_deser_tostring_file_read

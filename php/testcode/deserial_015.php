<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_file_upload_content
function deserial015(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('import');
    $content = file_get_contents($file['tmp_name']);
    $data = unserialize($content); // vuln-code-snippet vuln-line php_deser_file_upload_content
    return BenchmarkResponse::json(['imported' => count($data)]);
}
// vuln-code-snippet end php_deser_file_upload_content

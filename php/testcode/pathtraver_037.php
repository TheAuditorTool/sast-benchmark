<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_uuid_filename_discard
function pathtraver037(BenchmarkRequest $req): BenchmarkResponse {
    $ext = pathinfo($req->param('name'), PATHINFO_EXTENSION);
    $safeName = bin2hex(random_bytes(16)) . '.' . strtolower($ext);
    file_put_contents('/var/app/uploads/' . $safeName, $req->bodyStr()); // vuln-code-snippet safe-line php_pt_uuid_filename_discard
    return BenchmarkResponse::ok($safeName);
}
// vuln-code-snippet end php_pt_uuid_filename_discard

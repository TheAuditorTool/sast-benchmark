<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01204(BenchmarkRequest $req): BenchmarkResponse {
    $doc = $req->file('document');
    $ext = strtolower(pathinfo($doc['name'], PATHINFO_EXTENSION));
    $allowed = ['pdf', 'docx', 'xlsx'];
    if (!in_array($ext, $allowed, true)) {
        return BenchmarkResponse::badRequest('unsupported format');
    }
    $name = bin2hex(random_bytes(16)) . '.' . $ext;
    move_uploaded_file($doc['tmp_name'], '/srv/docs/' . $name);
    return BenchmarkResponse::ok('document saved');
}

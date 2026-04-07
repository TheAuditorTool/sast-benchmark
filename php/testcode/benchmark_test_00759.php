<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00759(BenchmarkRequest $req): BenchmarkResponse {
    $f = $req->param('file');
    $ext = pathinfo($f, PATHINFO_EXTENSION);
    $allowed_ext = ['jpg', 'png', 'gif'];
    if (!in_array(strtolower($ext), $allowed_ext, true)) {
        return BenchmarkResponse::badRequest('invalid extension');
    }
    $content = file_get_contents('/var/app/uploads/' . basename($f));
    return BenchmarkResponse::ok($content);
}

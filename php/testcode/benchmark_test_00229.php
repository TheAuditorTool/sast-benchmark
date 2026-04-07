<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00229(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $blocked = ['php', 'exe', 'sh', 'bat'];
    if (in_array($ext, $blocked, true)) {
        return BenchmarkResponse::badRequest('blocked extension');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}

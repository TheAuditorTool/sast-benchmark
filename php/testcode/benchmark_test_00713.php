<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00713(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $storedAs = bin2hex(random_bytes(16));
    move_uploaded_file($file['tmp_name'], '/var/uploads/' . $storedAs);
    $expires = time() + 3600;
    $sig     = hash_hmac('sha256', $storedAs . $expires, UPLOAD_SECRET);
    $url     = '/download/' . urlencode($storedAs) . "?exp={$expires}&sig={$sig}";
    return BenchmarkResponse::ok($url);
}

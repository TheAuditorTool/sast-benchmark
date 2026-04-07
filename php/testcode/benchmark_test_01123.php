<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01123(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    move_uploaded_file($file['tmp_name'], '/var/app/uploads/profile.jpg');
    return BenchmarkResponse::ok('Profile photo updated');
}

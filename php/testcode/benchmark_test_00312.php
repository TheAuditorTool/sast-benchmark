<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00312(BenchmarkRequest $req): BenchmarkResponse {
    $file   = $_FILES['f'];
    $key    = 'uploads/' . bin2hex(random_bytes(16));
    $s3     = $req->s3();
    $s3->putObject([
        'Bucket'               => 'app-uploads',
        'Key'                  => $key,
        'SourceFile'           => $file['tmp_name'],
        'ServerSideEncryption' => 'AES256',
    ]);
    $url = $s3->createPresignedRequest(
        $s3->getCommand('GetObject', ['Bucket' => 'app-uploads', 'Key' => $key]),
        '+1 hour'
    )->getUri();
    return BenchmarkResponse::ok((string) $url);
}

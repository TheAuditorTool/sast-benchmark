<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_s3_no_direct_path
function fileupload042(BenchmarkRequest $req): BenchmarkResponse {
    $file   = $_FILES['f'];
    $key    = 'uploads/' . bin2hex(random_bytes(16));
    $s3     = $req->s3();
    $s3->putObject([
        'Bucket'               => 'app-uploads',
        'Key'                  => $key,
        'SourceFile'           => $file['tmp_name'],
        'ServerSideEncryption' => 'AES256',
    ]); // vuln-code-snippet safe-line php_upload_s3_no_direct_path
    $url = $s3->createPresignedRequest(
        $s3->getCommand('GetObject', ['Bucket' => 'app-uploads', 'Key' => $key]),
        '+1 hour'
    )->getUri();
    return BenchmarkResponse::ok((string) $url);
}
// vuln-code-snippet end php_upload_s3_no_direct_path

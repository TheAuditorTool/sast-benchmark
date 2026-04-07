<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_finfo_os_mime
function fileupload035(BenchmarkRequest $req): BenchmarkResponse {
    $file  = $_FILES['f'];
    $finfo = finfo_open(FILEINFO_MIME_TYPE);
    $mime  = finfo_file($finfo, $file['tmp_name']); // vuln-code-snippet safe-line php_upload_finfo_os_mime
    finfo_close($finfo);
    if (!in_array($mime, ['image/jpeg', 'image/png', 'image/gif'], true)) {
        return BenchmarkResponse::badRequest('Invalid file type');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.' . pathinfo($file['name'], PATHINFO_EXTENSION);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_finfo_os_mime

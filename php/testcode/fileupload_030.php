<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_pdf_php_stream
function fileupload030(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if ($ext !== 'pdf') {
        return BenchmarkResponse::badRequest('Only PDF allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_pdf_php_stream
    return BenchmarkResponse::ok('PDF uploaded');
}
// vuln-code-snippet end php_upload_pdf_php_stream

<?php
// Symfony-style Search Controller
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start sy_sqli_qb_raw
function sy_sqli_qb_raw(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('q');
    $result = $pdo->query("SELECT * FROM users WHERE name LIKE '%" . $search . "%'"); // vuln-code-snippet vuln-line sy_sqli_qb_raw
    return BenchmarkResponse::json($result->fetchAll());
}
// vuln-code-snippet end sy_sqli_qb_raw

// vuln-code-snippet start sy_sqli_qb_param
function sy_sqli_qb_param(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('q');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE name LIKE ?"); // vuln-code-snippet safe-line sy_sqli_qb_param
    $stmt->execute(['%' . $search . '%']);
    return BenchmarkResponse::json($stmt->fetchAll());
}
// vuln-code-snippet end sy_sqli_qb_param

// vuln-code-snippet start sy_ssti_from_string
function sy_ssti_from_string(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $output = eval('return ' . $template . ';'); // vuln-code-snippet vuln-line sy_ssti_from_string
    return BenchmarkResponse::html((string)$output);
}
// vuln-code-snippet end sy_ssti_from_string

// vuln-code-snippet start sy_ssti_file
function sy_ssti_file(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['query' => htmlspecialchars($req->param('q'), ENT_QUOTES, 'UTF-8')];
    $templateFile = __DIR__ . '/../templates/search/results.html.twig'; // vuln-code-snippet safe-line sy_ssti_file
    $html = file_get_contents($templateFile);
    $html = str_replace('{{ query }}', $data['query'], $html);
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end sy_ssti_file

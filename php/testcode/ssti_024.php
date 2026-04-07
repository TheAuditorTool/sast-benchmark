<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_stored_db_template
function ssti024(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = new PDO('sqlite:/tmp/templates.db');
    $stmt = $db->prepare('SELECT body FROM templates WHERE id = ?');
    $stmt->execute([$id]);
    $tpl = $stmt->fetchColumn();
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader());
    $html = $twig->createTemplate($tpl)->render([]); // vuln-code-snippet vuln-line php_ssti_stored_db_template
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_stored_db_template

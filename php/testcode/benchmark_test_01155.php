<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01155(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $dept = $req->param('department');
    $minSalary = $req->param('min_salary');
    $stmt = $pdo->prepare("SELECT name, salary FROM employees WHERE department = ? AND salary >= ?");
    $stmt->execute([$dept, $minSalary]);
    return BenchmarkResponse::json($stmt->fetchAll());
}

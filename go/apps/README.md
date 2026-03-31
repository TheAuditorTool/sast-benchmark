# Go Reference App Benchmarks

## Overview

5 intentionally vulnerable Go applications with per-function ground truth classifications. Each app has a `ground_truth.csv` mapping every security-relevant function to its CWE category and vulnerable/safe status.

These apps complement the standalone `testcode/` benchmark by testing SAST tools against realistic multi-file Go projects with cross-function taint flows, multiple frameworks, and mixed vulnerable/safe patterns.

## Apps

| App | Framework | Files | Ground Truth Entries | Primary Patterns |
|-----|-----------|-------|---------------------|------------------|
| multi-api | gin, echo, chi, fiber, net/http | 13 | 114 | Multi-framework SQLi, CmdI, PathTrav, XSS, cross-file flows |
| calorie-tracker | gin, GORM | 30 | 137 | GORM Raw() vs Where() discrimination |
| go_notifications | gorilla/mux | 16 | 87 | SQLi, SSRF, CmdI, template injection, log injection |
| beego_admin | beego | 5 | 32 | Multi-hop controller->service chains, column injection |
| grpc_users | gRPC | 3 | 24 | Protobuf field taint, arbitrary SQL execution |
| cobra_cli_test | cobra | 1 | 0 | No attack surface (excluded) |

## Ground Truth Format

Each `ground_truth.csv` uses this format:

```csv
# function,file,start_line,end_line,category,vulnerable,CWE
GetUser,internal/handlers/gin_handlers.go,39,55,sqli,true,89
GetUserAlt,internal/handlers/gin_handlers.go,306,319,sqli,false,89
```

## How to Score

1. Run your SAST tool on an individual app directory
2. For each finding the tool produces (file + line), check if it falls within a `ground_truth.csv` line range
3. If the finding is within a `vulnerable=true` range: **True Positive**
4. If the finding is within a `vulnerable=false` range: **False Positive**
5. If no finding falls within a `vulnerable=true` range: **False Negative**
6. If no finding falls within a `vulnerable=false` range: **True Negative**

## What Makes These Valuable

Unlike the standalone `testcode/` tests (one function per file), these apps test:

- **Cross-file taint propagation**: Handler -> Service -> Repository chains where taint must be tracked across 2-3 files
- **Framework-specific sources**: Beego's `c.GetString()`, gorilla's `mux.Vars()`, gRPC's `req.Field` -- real framework APIs
- **Mixed codebases**: The same app has both vulnerable and safe patterns. The tool must distinguish them.
- **Real application architecture**: MVC layers, middleware, route registration, model definitions
- **Multi-hop chains**: 3+ function calls between source and sink

## Categories

| Category | CWE | Description |
|----------|-----|-------------|
| sqli | 89 | SQL Injection |
| cmdi | 78 | OS Command Injection |
| pathtraver | 22 | Path Traversal |
| xss | 79 | Cross-Site Scripting |
| ssrf | 918 | Server-Side Request Forgery |
| loginjection | 117 | Log Injection |
| cors | 346 | CORS Misconfiguration |

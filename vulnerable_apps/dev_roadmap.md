# Vulnerable Apps Benchmark -- Development Roadmap

> **Version:** 0.1.0 (2026-04-08)
> **Status:** Initial release. 26 apps, 884 ground truth entries across 5 languages.

---

## Planned

- [ ] App-level SARIF scorer (match SARIF findings to annotation line ranges, not just filenames)
- [ ] Per-app scorecard generation (TP/FP/FN/TN per app, not just per language)
- [ ] Cross-file taint scoring (mark multi-hop chains where source and sink are in different files)
- [ ] Java and Python reference apps (align with OWASP BenchmarkJava/BenchmarkPython)
- [ ] Go apps: expand cobra_cli_test with CLI injection patterns or remove from inventory

## Future Considerations

- Polyglot apps (anarchy_commerce already has Rust + TypeScript + Python -- could test cross-language taint)
- Container/infrastructure vulns (Dockerfile misconfigurations, docker-compose secrets)
- CI/CD pipeline injection patterns (GitHub Actions, GitLab CI)

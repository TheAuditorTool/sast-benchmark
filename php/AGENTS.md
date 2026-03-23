<!-- THEAUDITOR:START -->
# TheAuditor Agent System

**Start every task with:** `/theauditor:start` or `aud blueprint --structure`

## Quick Route
| Intent | Slash Command |
|--------|---------------|
| Any task (orchestrator) | `/theauditor:start` |
| Plan changes | `/theauditor:planning` |
| Refactor code | `/theauditor:refactor` |
| Security audit | `/theauditor:security` |
| Trace dataflow | `/theauditor:dataflow` |
| Assess impact | `/theauditor:impact` |

## Command Cheat Sheet
| Need | Command |
|------|---------|
| Architecture overview | `aud blueprint --structure` (ALWAYS FIRST) |
| List symbols in file | `aud query --file X --list all` |
| Who calls this? | `aud query --symbol X --show-callers` |
| Validation boundaries | `aud boundaries` |
| Dead code | `aud deadcode` |
| Blast radius | `aud impact --symbol X` |
| Full context | `aud explain path/to/file.py` |

## The Rules
1. **Database First** - Query before reading files
2. **Check flags** - Run `aud <cmd> --help` before guessing
3. **Cite evidence** - Every claim backed by query result
4. **Autonomous** - Execute commands, don't ask permission

**Full docs:** @/.auditor_venv/.theauditor_tools/agents/AGENTS.md
**Reinstall:** `aud setup-ai --target . --sync`
<!-- THEAUDITOR:END -->


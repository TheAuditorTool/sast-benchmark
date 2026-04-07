require_relative 'shared'

# vuln-code-snippet start ruby_fi_pathname_within_base
def handler(req)
  base = File.expand_path('modules')
  rel  = req.param('mod')
  p    = Pathname.new(base).join(rel + '.rb').cleanpath
  raise 'path traversal detected' unless p.to_s.start_with?(base) # vuln-code-snippet safe-line ruby_fi_pathname_within_base
  load(p.to_s)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_pathname_within_base

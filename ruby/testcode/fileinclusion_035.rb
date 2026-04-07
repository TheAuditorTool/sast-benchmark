require_relative 'shared'

# vuln-code-snippet start ruby_fi_multihop_fi
def handler(req)
  # Strips spaces but path traversal via '..' is still possible
  fname = req.param('file').gsub(' ', '_')
  load(File.join('modules', fname + '.rb')) # vuln-code-snippet vuln-line ruby_fi_multihop_fi
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_multihop_fi

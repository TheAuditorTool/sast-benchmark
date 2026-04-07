require_relative '../../testcode/shared'

# vuln-code-snippet start si_massassign_hash_all
def si_massassign_hash_all(req)
  attributes = req.post_data  # vuln-code-snippet vuln-line si_massassign_hash_all
  user = FakeActiveRecord::Base.where(attributes)
  BenchmarkResponse.json({ status: 'created', user: user.to_a.first.to_s })
end
# vuln-code-snippet end si_massassign_hash_all

# vuln-code-snippet start si_massassign_slice
def si_massassign_slice(req)
  allowed_keys = %w[name email role]
  attributes = req.post_data.slice(*allowed_keys)  # vuln-code-snippet safe-line si_massassign_slice
  user = FakeActiveRecord::Base.where(attributes)
  BenchmarkResponse.json({ status: 'created', user: user.to_a.first.to_s })
end
# vuln-code-snippet end si_massassign_slice

# vuln-code-snippet start si_cmdi_system_concat
def si_cmdi_system_concat(req)
  filename = req.param('filename')
  system("convert " + filename + " output.png")  # vuln-code-snippet vuln-line si_cmdi_system_concat
  BenchmarkResponse.ok('conversion complete')
end
# vuln-code-snippet end si_cmdi_system_concat

# vuln-code-snippet start si_cmdi_exec_array
def si_cmdi_exec_array(req)
  filename = req.param('filename')
  system("convert", filename, "output.png")  # vuln-code-snippet safe-line si_cmdi_exec_array
  BenchmarkResponse.ok('conversion complete')
end
# vuln-code-snippet end si_cmdi_exec_array

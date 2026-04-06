require_relative 'shared'

# vuln-code-snippet start ruby_pt_cleanpath_check
def read_safe(req)
  input = req.param('file')
  base = '/var/data'
  full = Pathname.new(File.join(base, input)).cleanpath.to_s
  return BenchmarkResponse.bad_request('outside base') unless full.start_with?(base) # vuln-code-snippet safe-line ruby_pt_cleanpath_check
  BenchmarkResponse.ok(File.read(full))
end
# vuln-code-snippet end ruby_pt_cleanpath_check

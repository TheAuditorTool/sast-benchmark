require_relative 'shared'

require 'pathname'

# vuln-code-snippet start ruby_pt_cleanpath
def serve_static_file(req)
  base = "/var/www/static"
  user_path = req.param('path')
  clean = Pathname.new(base).join(user_path).cleanpath.to_s
  return BenchmarkResponse.bad_request("Access denied") unless clean.start_with?(base + "/") # vuln-code-snippet safe-line ruby_pt_cleanpath
  content = File.read(clean)
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_cleanpath

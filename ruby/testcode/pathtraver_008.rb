require_relative 'shared'

require 'pathname'

# vuln-code-snippet start ruby_pt_realpath_check
def read_user_file(req)
  allowed_dir = "/var/app/storage"
  path = req.param('path')
  real = Pathname.new(File.join(allowed_dir, path)).realpath.to_s
  return BenchmarkResponse.bad_request("Access denied") unless real.start_with?(allowed_dir + "/") # vuln-code-snippet safe-line ruby_pt_realpath_check
  content = File.read(real)
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_realpath_check

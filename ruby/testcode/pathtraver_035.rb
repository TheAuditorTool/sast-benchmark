require_relative 'shared'

SAFE_ROOT = '/app/files'.freeze

# vuln-code-snippet start ruby_pt_dir_chroot
def read_with_chdir(req)
  fname = File.basename(req.param('file'))
  content = Dir.chdir(SAFE_ROOT) { File.read(fname) } # vuln-code-snippet safe-line ruby_pt_dir_chroot
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_dir_chroot

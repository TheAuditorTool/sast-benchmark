require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_pty_spawn_array
def convert_image_safe(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  output_lines = []
  PTY.spawn('convert', filename, '/tmp/output.png') do |r, _w, _pid|  # vuln-code-snippet safe-line ruby_cmdi_pty_spawn_array
    output_lines << r.readline rescue nil
  end
  BenchmarkResponse.json({ output: output_lines.join })
end
# vuln-code-snippet end ruby_cmdi_pty_spawn_array

require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_pty_spawn
def convert_image(req)
  filename = req.param('file')
  output_lines = []
  PTY.spawn("convert #{filename} /tmp/output.png") do |r, _w, _pid|  # vuln-code-snippet vuln-line ruby_cmdi_pty_spawn
    output_lines << r.readline rescue nil
  end
  BenchmarkResponse.json({ output: output_lines.join })
end
# vuln-code-snippet end ruby_cmdi_pty_spawn

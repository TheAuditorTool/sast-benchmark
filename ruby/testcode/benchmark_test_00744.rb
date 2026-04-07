require_relative 'shared'

def convert_image_safe(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  output_lines = []
  PTY.spawn('convert', filename, '/tmp/output.png') do |r, _w, _pid|
    output_lines << r.readline rescue nil
  end
  BenchmarkResponse.json({ output: output_lines.join })
end

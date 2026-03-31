require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_spawn_interp
def transcode_video(req)
  input_file = req.post('input_file')
  pid = spawn("ffmpeg -i #{input_file} -o out.mp4") # vuln-code-snippet vuln-line ruby_cmdi_spawn_interp
  Process.detach(pid)
  BenchmarkResponse.ok("transcoding started with pid #{pid}")
end
# vuln-code-snippet end ruby_cmdi_spawn_interp

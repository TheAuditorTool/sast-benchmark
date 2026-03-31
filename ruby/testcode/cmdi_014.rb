require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_spawn_array
def transcode_video_safe(req)
  input_file = req.post('input_file')
  pid = spawn("ffmpeg", "-i", input_file, "-o", "out.mp4") # vuln-code-snippet safe-line ruby_cmdi_spawn_array
  Process.detach(pid)
  BenchmarkResponse.ok("transcoding started with pid #{pid}")
end
# vuln-code-snippet end ruby_cmdi_spawn_array

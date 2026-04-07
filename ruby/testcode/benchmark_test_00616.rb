require_relative 'shared'

def transcode_video_safe(req)
  input_file = req.post('input_file')
  pid = spawn("ffmpeg", "-i", input_file, "-o", "out.mp4")
  Process.detach(pid)
  BenchmarkResponse.ok("transcoding started with pid #{pid}")
end
